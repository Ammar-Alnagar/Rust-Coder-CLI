use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::process::Command;
use crate::llm::{self, Message};
use crate::config::LlmConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "tool", content = "parameters")]
pub enum Tool {
    ReadFile { path: String },
    WriteFile { path: String, content: String },
    RunCommand { command: String },
}

impl Tool {
    pub fn execute(&self) -> Result<String, io::Error> {
        match self {
            Tool::ReadFile { path } => {
                fs::read_to_string(path)
            }
            Tool::WriteFile { path, content } => {
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
                    Ok(String::from_utf8_lossy(&output.stderr).to_string())
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
        let system_prompt = Message {
            role: "system".to_string(),
            content: r#"You are a helpful assistant with access to the following tools. To use a tool, respond with a JSON object with the "tool" and "parameters" fields.

Available tools:
- ReadFile: Reads the content of a file.
  - parameters: {"path": "path/to/file"}
- WriteFile: Writes content to a file.
  - parameters: {"path": "path/to/file", "content": "file content"}
- RunCommand: Executes a shell command.
  - parameters: {"command": "your command"}

Example of using the ReadFile tool:
{"tool": "ReadFile", "parameters": {"path": "src/main.rs"}}

If you don't need to use a tool, just respond with a regular message."#.to_string(),
        };

        Self {
            messages: vec![system_prompt],
        }
    }

    pub async fn run(&mut self, config: &LlmConfig, user_prompt: String) -> Result<String, Box<dyn std::error::Error>> {
        self.messages.push(Message {
            role: "user".to_string(),
            content: user_prompt,
        });

        loop {
            let response = llm::ask_llm(config, self.messages.clone()).await?;

            if let Ok(tool) = serde_json::from_str::<Tool>(&response) {
                let tool_result = tool.execute()?;
                self.messages.push(Message {
                    role: "assistant".to_string(),
                    content: response,
                });
                self.messages.push(Message {
                    role: "tool".to_string(),
                    content: tool_result,
                });
            } else {
                self.messages.push(Message {
                    role: "assistant".to_string(),
                    content: response.clone(),
                });
                return Ok(response);
            }
        }
    }
}
