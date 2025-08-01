use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::process::Command;
use std::path::Path;
use crate::llm::{self, Message};
use crate::config::LlmConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Tool {
    ReadFile { path: String },
    WriteFile { path: String, content: String },
    RunCommand { command: String },
    ListFiles { path: String },
    CreateDirectory { path: String },
    DeleteFile { path: String },
    ExecuteCode { language: String, code: String },
}

impl Tool {
    pub fn execute(&self) -> Result<String, io::Error> {
        match self {
            Tool::ReadFile { path } => {
                fs::read_to_string(path)
            }
            Tool::WriteFile { path, content } => {
                // Create parent directories if they don't exist
                if let Some(parent) = Path::new(path).parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(path, content)?;
                Ok(format!("File '{}' written successfully.", path))
            }
            Tool::RunCommand { command } => {
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .output()?;

                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Ok(format!("Error: {}", String::from_utf8_lossy(&output.stderr)))
                }
            }
            Tool::ListFiles { path } => {
                let entries = fs::read_dir(path)?;
                let mut files = Vec::new();
                for entry in entries {
                    let entry = entry?;
                    let name = entry.file_name().to_string_lossy().to_string();
                    let is_dir = entry.file_type()?.is_dir();
                    files.push(format!("{}{}", if is_dir { "[DIR] " } else { "" }, name));
                }
                Ok(files.join("\n"))
            }
            Tool::CreateDirectory { path } => {
                fs::create_dir_all(path)?;
                Ok(format!("Directory '{}' created successfully.", path))
            }
            Tool::DeleteFile { path } => {
                if Path::new(path).is_dir() {
                    fs::remove_dir_all(path)?;
                } else {
                    fs::remove_file(path)?;
                }
                Ok(format!("'{}' deleted successfully.", path))
            }
            Tool::ExecuteCode { language, code } => {
                match language.to_lowercase().as_str() {
                    "python" | "py" => {
                        let temp_file = format!("/tmp/temp_code_{}.py", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
                        fs::write(&temp_file, code)?;
                        let output = Command::new("python3")
                            .arg(&temp_file)
                            .output()?;
                        let _ = fs::remove_file(temp_file); // Clean up
                        if output.status.success() {
                            Ok(String::from_utf8_lossy(&output.stdout).to_string())
                        } else {
                            Ok(format!("Error: {}", String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "bash" | "sh" => {
                        let output = Command::new("bash")
                            .arg("-c")
                            .arg(&code)
                            .output()?;
                        if output.status.success() {
                            Ok(String::from_utf8_lossy(&output.stdout).to_string())
                        } else {
                            Ok(format!("Error: {}", String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "rust" => {
                        let temp_dir = format!("/tmp/rust_code_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
                        fs::create_dir_all(&temp_dir)?;
                        let main_rs = format!("{}/src/main.rs", temp_dir);
                        let cargo_toml = format!("{}/Cargo.toml", temp_dir);
                        
                        // Create Cargo.toml
                        fs::write(&cargo_toml, r#"[package]
name = "temp_code"
version = "0.1.0"
edition = "2021"

[dependencies]
"#)?;
                        
                        // Create src directory and main.rs
                        fs::create_dir_all(format!("{}/src", temp_dir))?;
                        fs::write(&main_rs, code)?;
                        
                        let output = Command::new("cargo")
                            .arg("run")
                            .current_dir(&temp_dir)
                            .output()?;
                        
                        let _ = fs::remove_dir_all(temp_dir); // Clean up
                        if output.status.success() {
                            Ok(String::from_utf8_lossy(&output.stdout).to_string())
                        } else {
                            Ok(format!("Error: {}", String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    _ => Ok(format!("Unsupported language: {}. Supported: python, bash, rust", language))
                }
            }
        }
    }
}

pub struct Agent {
    messages: Vec<Message>,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            messages: vec![],
        }
    }

    fn get_system_prompt() -> String {
        r#"You are a helpful AI assistant with access to various tools. You MUST use tools to complete tasks - do not just describe what you would do.

AVAILABLE TOOLS:
1. READ_FILE <path> - Read the contents of a file
2. WRITE_FILE <path> <content> - Create or modify a file with the specified content
3. RUN_COMMAND <command> - Execute shell commands
4. LIST_FILES <path> - List files in a directory
5. CREATE_DIRECTORY <path> - Create directories
6. DELETE_FILE <path> - Delete files or directories
7. EXECUTE_CODE <language> <code> - Execute code in Python, Bash, or Rust

CRITICAL RULES:
- ALWAYS use tools to complete tasks, never just describe actions
- Use the exact format: TOOL: <tool_name> <parameters>
- For WRITE_FILE, separate path and content with a space
- For EXECUTE_CODE, specify language first, then code
- If a task requires multiple steps, use tools for each step
- Verify results by using READ_FILE or LIST_FILES after creating/modifying files

EXAMPLES:
TOOL: READ_FILE /path/to/file.txt
TOOL: WRITE_FILE /path/to/file.txt This is the file content
TOOL: RUN_COMMAND ls -la
TOOL: LIST_FILES /path/to/directory
TOOL: CREATE_DIRECTORY /path/to/new/directory
TOOL: DELETE_FILE /path/to/file.txt
TOOL: EXECUTE_CODE python print("Hello World")

TASK COMPLETION STRATEGY:
1. Break complex tasks into steps
2. Use appropriate tools for each step
3. Verify results with READ_FILE or LIST_FILES
4. Continue until the task is fully completed
5. Provide a summary of what was accomplished

Remember: You have access to real tools - USE THEM to actually complete tasks, don't just describe what you would do."#.to_string()
    }

    fn parse_tool_call(&self, response: &str) -> Option<Tool> {
        let lines: Vec<&str> = response.lines().collect();
        for line in lines {
            if line.starts_with("TOOL:") {
                let parts: Vec<&str> = line[6..].trim().splitn(2, ' ').collect();
                if parts.len() >= 2 {
                    let tool_name = parts[0];
                    let params = parts[1];
                    
                    return match tool_name {
                        "READ_FILE" => Some(Tool::ReadFile { path: params.to_string() }),
                        "WRITE_FILE" => {
                            // Find the first space to separate path from content
                            if let Some(space_pos) = params.find(' ') {
                                let path = &params[..space_pos];
                                let content = &params[space_pos + 1..];
                                Some(Tool::WriteFile { 
                                    path: path.to_string(), 
                                    content: content.to_string() 
                                })
                            } else {
                                None
                            }
                        }
                        "RUN_COMMAND" => Some(Tool::RunCommand { command: params.to_string() }),
                        "LIST_FILES" => Some(Tool::ListFiles { path: params.to_string() }),
                        "CREATE_DIRECTORY" => Some(Tool::CreateDirectory { path: params.to_string() }),
                        "DELETE_FILE" => Some(Tool::DeleteFile { path: params.to_string() }),
                        "EXECUTE_CODE" => {
                            // Find the first space to separate language from code
                            if let Some(space_pos) = params.find(' ') {
                                let language = &params[..space_pos];
                                let code = &params[space_pos + 1..];
                                Some(Tool::ExecuteCode { 
                                    language: language.to_string(), 
                                    code: code.to_string() 
                                })
                            } else {
                                None
                            }
                        }
                        _ => None
                    };
                }
            }
        }
        None
    }

    pub async fn run(&mut self, config: &LlmConfig, user_prompt: String) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
        // Add system message if this is the first interaction
        if self.messages.is_empty() {
            self.messages.push(Message {
                role: "system".to_string(),
                content: Self::get_system_prompt(),
            });
        }

        // Add user message
        self.messages.push(Message {
            role: "user".to_string(),
            content: user_prompt.clone(),
        });

        let mut all_tool_logs = Vec::new();
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 5;

        loop {
            attempts += 1;
            
            // Get response from LLM
            let response = llm::ask_llm_with_messages(config, &self.messages).await?;

            // Check if response contains a tool call
            if let Some(tool) = self.parse_tool_call(&response) {
                let mut tool_logs = Vec::new();
                
                // Log the tool execution
                let tool_name = match &tool {
                    Tool::ReadFile { path } => format!("READ_FILE {}", path),
                    Tool::WriteFile { path, content: _ } => format!("WRITE_FILE {}", path),
                    Tool::RunCommand { command } => format!("RUN_COMMAND {}", command),
                    Tool::ListFiles { path } => format!("LIST_FILES {}", path),
                    Tool::CreateDirectory { path } => format!("CREATE_DIRECTORY {}", path),
                    Tool::DeleteFile { path } => format!("DELETE_FILE {}", path),
                    Tool::ExecuteCode { language, code: _ } => format!("EXECUTE_CODE {}", language),
                };
                tool_logs.push(format!("🔧 Attempt {}: Executing {}", attempts, tool_name));
                
                // Execute the tool
                let tool_result = tool.execute()?;
                tool_logs.push(format!("✅ Result: {}", tool_result));
                all_tool_logs.extend(tool_logs);
                
                // Add assistant message and tool result to conversation
                self.messages.push(Message {
                    role: "assistant".to_string(),
                    content: response,
                });
                
                self.messages.push(Message {
                    role: "user".to_string(),
                    content: format!("Tool result: {}", tool_result),
                });

                // Check if we should continue or if the task is complete
                if attempts >= MAX_ATTEMPTS {
                    // Get final response after max attempts
                    let final_response = llm::ask_llm_with_messages(config, &self.messages).await?;
                    self.messages.push(Message {
                        role: "assistant".to_string(),
                        content: final_response.clone(),
                    });
                    
                    all_tool_logs.push(format!("⚠️  Reached maximum attempts ({})", MAX_ATTEMPTS));
                    return Ok((final_response, all_tool_logs));
                }

                // Continue to next iteration to see if more tools are needed
                continue;
            } else {
                // No tool call, task appears to be complete
                self.messages.push(Message {
                    role: "assistant".to_string(),
                    content: response.clone(),
                });
                
                if attempts > 1 {
                    all_tool_logs.push(format!("✅ Task completed after {} attempts", attempts));
                }
                
                return Ok((response, all_tool_logs));
            }
        }
    }
}
