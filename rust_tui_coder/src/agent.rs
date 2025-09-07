use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::process::Command;
use std::path::Path;
use crate::llm::{self, Message};
use crate::config::LlmConfig;

#[derive(Serialize, Deserialize, Debug)]
struct ToolCall {
    name: String,
    parameters: serde_json::Value,
}

impl ToolCall {
    fn into_tool(self) -> Option<Tool> {
        match self.name.as_str() {
            "READ_FILE" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                Some(Tool::ReadFile { path })
            }
            "WRITE_FILE" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                let content = self.parameters.get("content")?.as_str()?.to_string();
                Some(Tool::WriteFile { path, content })
            }
            "APPEND_FILE" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                let content = self.parameters.get("content")?.as_str()?.to_string();
                Some(Tool::AppendFile { path, content })
            }
            "SEARCH_REPLACE" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                let old_string = self.parameters.get("old_string")?.as_str()?.to_string();
                let new_string = self.parameters.get("new_string")?.as_str()?.to_string();
                Some(Tool::SearchReplace { path, old_string, new_string })
            }
            "DELETE_FILE" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                Some(Tool::DeleteFile { path })
            }
            "LIST_FILES" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                Some(Tool::ListFiles { path })
            }
            "LIST_FILES_RECURSIVE" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                Some(Tool::ListFilesRecursive { path })
            }
            "CREATE_DIRECTORY" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                Some(Tool::CreateDirectory { path })
            }
            "GREP_SEARCH" => {
                let pattern = self.parameters.get("pattern")?.as_str()?.to_string();
                let path = self.parameters.get("path").and_then(|p| p.as_str()).map(|s| s.to_string());
                Some(Tool::GrepSearch { pattern, path })
            }
            "GLOB_SEARCH" => {
                let pattern = self.parameters.get("pattern")?.as_str()?.to_string();
                Some(Tool::GlobSearch { pattern })
            }
            "EXECUTE_CODE" => {
                let language = self.parameters.get("language")?.as_str()?.to_string();
                let code = self.parameters.get("code")?.as_str()?.to_string();
                Some(Tool::ExecuteCode { language, code })
            }
            "RUN_COMMAND" => {
                let command = self.parameters.get("command")?.as_str()?.to_string();
                Some(Tool::RunCommand { command })
            }
            "GIT_STATUS" => Some(Tool::GitStatus),
            "GIT_DIFF" => Some(Tool::GitDiff),
            "GIT_COMMIT" => {
                let message = self.parameters.get("message")?.as_str()?.to_string();
                Some(Tool::GitCommit { message })
            }
            "GIT_LOG" => {
                let count = self.parameters.get("count").and_then(|c| c.as_u64()).map(|c| c as usize);
                Some(Tool::GitLog { count })
            }
            "RUN_LINT" => {
                let language = self.parameters.get("language")?.as_str()?.to_string();
                let path = self.parameters.get("path").and_then(|p| p.as_str()).map(|s| s.to_string());
                Some(Tool::RunLint { language, path })
            }
            "RUN_TESTS" => {
                let framework = self.parameters.get("framework")?.as_str()?.to_string();
                let path = self.parameters.get("path").and_then(|p| p.as_str()).map(|s| s.to_string());
                Some(Tool::RunTests { framework, path })
            }
            "INSTALL_PACKAGE" => {
                let manager = self.parameters.get("manager")?.as_str()?.to_string();
                let package = self.parameters.get("package")?.as_str()?.to_string();
                Some(Tool::InstallPackage { manager, package })
            }
            "CREATE_PLAN" => {
                let task = self.parameters.get("task")?.as_str()?.to_string();
                let steps_array = self.parameters.get("steps")?.as_array()?;
                let steps: Vec<String> = steps_array.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect();
                Some(Tool::CreatePlan { task, steps })
            }
            "UPDATE_PLAN" => {
                let completed_step = self.parameters.get("completed_step")?.as_u64()? as usize;
                Some(Tool::UpdatePlan { completed_step })
            }
            "CLEAR_PLAN" => Some(Tool::ClearPlan),
            _ => None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Tool {
    // File Operations
    ReadFile { path: String },
    WriteFile { path: String, content: String },
    AppendFile { path: String, content: String },
    SearchReplace { path: String, old_string: String, new_string: String },
    DeleteFile { path: String },

    // Directory Operations
    ListFiles { path: String },
    ListFilesRecursive { path: String },
    CreateDirectory { path: String },

    // Search & Navigation
    GrepSearch { pattern: String, path: Option<String> },
    GlobSearch { pattern: String },

    // Code Execution & Compilation
    ExecuteCode { language: String, code: String },
    RunCommand { command: String },

    // Development Workflow
    GitStatus,
    GitDiff,
    GitCommit { message: String },
    GitLog { count: Option<usize> },

    // Quality Assurance
    RunLint { language: String, path: Option<String> },
    RunTests { framework: String, path: Option<String> },

    // Package Management
    InstallPackage { manager: String, package: String },

    // Planning and Task Management
    CreatePlan { task: String, steps: Vec<String> },
    UpdatePlan { completed_step: usize },
    ClearPlan,
}

impl Tool {
    pub fn execute(&self) -> Result<String, io::Error> {
        match self {
            // File Operations
            Tool::ReadFile { path } => {
                let content = fs::read_to_string(path)?;
                Ok(format!("File: {}\n\n{}", path, content))
            }
            Tool::WriteFile { path, content } => {
                // Create parent directories if they don't exist
                if let Some(parent) = Path::new(path).parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(path, content)?;
                Ok(format!("File '{}' written successfully ({} bytes).", path, content.len()))
            }
            Tool::AppendFile { path, content } => {
                if !Path::new(path).exists() {
                    return Err(io::Error::new(io::ErrorKind::NotFound, format!("File '{}' does not exist", path)));
                }
                let mut file = fs::OpenOptions::new().append(true).open(path)?;
                use std::io::Write;
                file.write_all(content.as_bytes())?;
                Ok(format!("Content appended to '{}' successfully ({} bytes added).", path, content.len()))
            }
            Tool::SearchReplace { path, old_string, new_string } => {
                let content = fs::read_to_string(path)?;
                if !content.contains(old_string) {
                    return Err(io::Error::new(io::ErrorKind::InvalidData,
                        format!("Old string '{}' not found in file '{}'", old_string, path)));
                }
                let new_content = content.replace(old_string, new_string);
                fs::write(path, new_content)?;
                Ok(format!("Successfully replaced '{}' with '{}' in '{}'", old_string, new_string, path))
            }
            Tool::DeleteFile { path } => {
                if Path::new(path).is_dir() {
                    fs::remove_dir_all(path)?;
                    Ok(format!("Directory '{}' deleted successfully.", path))
                } else {
                    fs::remove_file(path)?;
                    Ok(format!("File '{}' deleted successfully.", path))
                }
            }

            // Directory Operations
            Tool::ListFiles { path } => {
                let entries = fs::read_dir(path)?;
                let mut files = Vec::new();
                for entry in entries {
                    let entry = entry?;
                    let name = entry.file_name().to_string_lossy().to_string();
                    let is_dir = entry.file_type()?.is_dir();
                    files.push(format!("{}{}", if is_dir { "[DIR] " } else { "[FILE] " }, name));
                }
                files.sort();
                Ok(format!("Contents of '{}':\n{}", path, files.join("\n")))
            }
            Tool::ListFilesRecursive { path } => {
                fn collect_files(dir: &Path, prefix: &str) -> Result<Vec<String>, io::Error> {
                    let mut results = Vec::new();
                    let entries = fs::read_dir(dir)?;
                    for entry in entries {
                        let entry = entry?;
                        let name = entry.file_name().to_string_lossy().to_string();
                        let full_path = entry.path();
                        let is_dir = entry.file_type()?.is_dir();

                        if is_dir {
                            results.push(format!("{}[DIR] {}{}", prefix, name, std::path::MAIN_SEPARATOR));
                            let sub_results = collect_files(&full_path, &format!("{}  ", prefix))?;
                            results.extend(sub_results);
                        } else {
                            results.push(format!("{}[FILE] {}", prefix, name));
                        }
                    }
                    Ok(results)
                }
                let files = collect_files(Path::new(path), "")?;
                Ok(format!("Recursive contents of '{}':\n{}", path, files.join("\n")))
            }
            Tool::CreateDirectory { path } => {
                fs::create_dir_all(path)?;
                Ok(format!("Directory '{}' created successfully.", path))
            }

            // Search & Navigation
            Tool::GrepSearch { pattern, path } => {
                let search_path = path.as_ref().map(|s| s.as_str()).unwrap_or(".");
                let mut cmd = Command::new("grep");
                cmd.arg("-r").arg("-n").arg(pattern).arg(search_path);
                let output = cmd.output()?;

                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Ok(format!("No matches found for pattern '{}' in '{}'", pattern, search_path))
                }
            }
            Tool::GlobSearch { pattern } => {
                let mut cmd = Command::new("find");
                cmd.arg(".").arg("-name").arg(pattern);
                let output = cmd.output()?;

                if output.status.success() {
                    let stdout_str = String::from_utf8_lossy(&output.stdout);
                    let files_vec: Vec<_> = stdout_str
                        .lines()
                        .filter(|line| !line.is_empty())
                        .collect();
                    Ok(format!("Files matching '{}':\n{}", pattern, files_vec.join("\n")))
                } else {
                    Ok(format!("No files found matching pattern '{}'", pattern))
                }
            }

            // Code Execution & Compilation
            Tool::ExecuteCode { language, code } => {
                match language.to_lowercase().as_str() {
                    "python" | "py" => Self::execute_python(code),
                    "javascript" | "js" | "node" => Self::execute_javascript(code),
                    "bash" | "sh" => Self::execute_bash(code),
                    "rust" => Self::execute_rust(code),
                    "go" => Self::execute_go(code),
                    "java" => Self::execute_java(code),
                    "c" | "cpp" | "c++" => Self::execute_c_cpp(code, language),
                    _ => Ok(format!("Unsupported language: {}. Supported: python, javascript, bash, rust, go, java, c, cpp", language))
                }
            }
            Tool::RunCommand { command } => {
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(command)
                            .output()?;

                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if stdout.is_empty() && stderr.is_empty() {
                        Ok(format!("Command '{}' executed successfully.", command))
                    } else {
                        Ok(format!("Command: {}\n\nSTDOUT:\n{}\n\nSTDERR:\n{}", command, stdout, stderr))
                    }
                } else {
                    Ok(format!("Command '{}' failed:\n\nSTDERR:\n{}", command, String::from_utf8_lossy(&output.stderr)))
                }
            }

            // Development Workflow
            Tool::GitStatus => {
                let output = Command::new("git").arg("status").arg("--porcelain").output()?;
                if output.status.success() {
                    let status = String::from_utf8_lossy(&output.stdout);
                    if status.trim().is_empty() {
                        Ok("Git repository is clean - no changes to commit.".to_string())
                    } else {
                        Ok(format!("Git status:\n{}", status))
                    }
                } else {
                    Ok("Not a git repository or git command failed.".to_string())
                }
            }
            Tool::GitDiff => {
                let output = Command::new("git").arg("diff").output()?;
                if output.status.success() {
                    let diff = String::from_utf8_lossy(&output.stdout);
                    if diff.trim().is_empty() {
                        Ok("No unstaged changes.".to_string())
                    } else {
                        Ok(format!("Git diff:\n{}", diff))
                    }
                } else {
                    Ok("Failed to get git diff.".to_string())
                }
            }
            Tool::GitCommit { message } => {
                // First add all changes
                let _ = Command::new("git").arg("add").arg(".").output()?;
                let output = Command::new("git").arg("commit").arg("-m").arg(message).output()?;
                if output.status.success() {
                    Ok(format!("Successfully committed changes with message: '{}'", message))
                } else {
                    Ok(format!("Commit failed: {}", String::from_utf8_lossy(&output.stderr)))
                }
            }
            Tool::GitLog { count } => {
                let mut cmd = Command::new("git");
                cmd.arg("log").arg("--oneline");
                if let Some(n) = count {
                    cmd.arg(format!("-{}", n));
                }
                let output = cmd.output()?;
                if output.status.success() {
                    Ok(format!("Git log:\n{}", String::from_utf8_lossy(&output.stdout)))
                } else {
                    Ok("Failed to get git log.".to_string())
                }
            }

            // Quality Assurance
            Tool::RunLint { language, path } => {
                let target_path = path.as_ref().map(|s| s.as_str()).unwrap_or(".");
                match language.to_lowercase().as_str() {
                    "rust" => {
                        let mut cmd = Command::new("cargo");
                        cmd.arg("clippy").current_dir(target_path);
                        let output = cmd.output()?;
                        if output.status.success() {
                            Ok("Cargo clippy passed - no linting issues found.".to_string())
                        } else {
                            Ok(format!("Linting issues found:\n{}", String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "python" => {
                        let output = if let Ok(out) = Command::new("flake8").arg(target_path).output() {
                            out
                        } else if let Ok(out) = Command::new("pylint").arg(target_path).output() {
                            out
                        } else {
                            match Command::new("python").arg("-m").arg("pycodestyle").arg(target_path).output() {
                                Ok(out) => out,
                                Err(_) => std::process::Output {
                                    status: std::process::ExitStatus::default(),
                                    stdout: Vec::new(),
                                    stderr: b"Python linter not available".to_vec(),
                                }
                            }
                        };
                        if output.status.success() {
                            Ok("Python linting passed.".to_string())
                        } else {
                            Ok(format!("Python linting issues:\n{}", String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "javascript" | "js" => {
                        let output = match Command::new("eslint").arg(target_path).output() {
                            Ok(out) => out,
                            Err(_) => std::process::Output {
                                status: std::process::ExitStatus::default(),
                                stdout: Vec::new(),
                                stderr: b"ESLint not available".to_vec(),
                            }
                        };
                        if output.status.success() {
                            Ok("JavaScript linting passed.".to_string())
                        } else {
                            Ok(format!("JavaScript linting issues:\n{}", String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    _ => Ok(format!("Linting not supported for language: {}", language))
                }
            }
            Tool::RunTests { framework, path } => {
                let target_path = path.as_ref().map(|s| s.as_str()).unwrap_or(".");
                match framework.to_lowercase().as_str() {
                    "cargo" |                     "rust" => {
                        let mut cmd = Command::new("cargo");
                        cmd.arg("test").current_dir(target_path);
                        let output = cmd.output()?;
                        if output.status.success() {
                            Ok(format!("Tests passed!\n{}", String::from_utf8_lossy(&output.stdout)))
                        } else {
                            Ok(format!("Tests failed:\n{}", String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "pytest" | "python" => {
                        let mut cmd = Command::new("python");
                        cmd.arg("-m").arg("pytest").current_dir(target_path);
                        let output = if let Ok(out) = cmd.output() {
                            out
                        } else {
                            let mut cmd2 = Command::new("python");
                            cmd2.arg("-m").arg("unittest").arg("discover").current_dir(target_path);
                            match cmd2.output() {
                                Ok(out) => out,
                                Err(_) => std::process::Output {
                                    status: std::process::ExitStatus::default(),
                                    stdout: Vec::new(),
                                    stderr: b"Python testing framework not available".to_vec(),
                                }
                            }
                        };
                        if output.status.success() {
                            Ok(format!("Python tests passed!\n{}", String::from_utf8_lossy(&output.stdout)))
                        } else {
                            Ok(format!("Python tests failed:\n{}", String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "jest" | "javascript" | "js" => {
                        let mut cmd = Command::new("npx");
                        cmd.arg("jest").current_dir(target_path);
                        let output = match cmd.output() {
                            Ok(out) => out,
                            Err(_) => std::process::Output {
                                status: std::process::ExitStatus::default(),
                                stdout: Vec::new(),
                                stderr: b"Jest testing framework not available".to_vec(),
                            }
                        };
                        if output.status.success() {
                            Ok(format!("JavaScript tests passed!\n{}", String::from_utf8_lossy(&output.stdout)))
                        } else {
                            Ok(format!("JavaScript tests failed:\n{}", String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    _ => Ok(format!("Testing framework '{}' not supported.", framework))
                }
            }

            // Package Management
            Tool::InstallPackage { manager, package } => {
                match manager.to_lowercase().as_str() {
                    "npm" | "yarn" => {
                        let cmd = if manager == "yarn" { "yarn" } else { "npm" };
                        let output = Command::new(cmd).arg("install").arg(package).output()?;
                        if output.status.success() {
                            Ok(format!("Successfully installed {} package: {}", manager, package))
                        } else {
                            Ok(format!("Failed to install {} package '{}': {}", manager, package, String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "cargo" | "rust" => {
                        let output = Command::new("cargo").arg("add").arg(package).output()?;
                        if output.status.success() {
                            Ok(format!("Successfully added Rust crate: {}", package))
                        } else {
                            Ok(format!("Failed to add Rust crate '{}': {}", package, String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "pip" | "python" => {
                        let output = Command::new("pip").arg("install").arg(package).output()?;
                        if output.status.success() {
                            Ok(format!("Successfully installed Python package: {}", package))
                        } else {
                            Ok(format!("Failed to install Python package '{}': {}", package, String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "go" => {
                        let output = Command::new("go").arg("get").arg(package).output()?;
                        if output.status.success() {
                            Ok(format!("Successfully installed Go package: {}", package))
                        } else {
                            Ok(format!("Failed to install Go package '{}': {}", package, String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    _ => Ok(format!("Package manager '{}' not supported.", manager))
                }
            }

            // Planning and Task Management
            Tool::CreatePlan { task, steps } => {
                let mut content = format!("# Task Plan: {}\n\n", task);
                content.push_str("## Checklist:\n\n");
                for (i, step) in steps.iter().enumerate() {
                    content.push_str(&format!("- [ ] Step {}: {}\n", i + 1, step));
                }
                content.push_str("\n## Progress:\n");
                content.push_str("- Total Steps: 0\n");
                content.push_str("- Completed: 0\n");
                content.push_str("- Remaining: 0\n");

                fs::write("plan.md", content)?;
                Ok(format!("Created plan.md with {} steps for task: {}", steps.len(), task))
            }

            Tool::UpdatePlan { completed_step } => {
                let content = fs::read_to_string("plan.md")?;
                let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

                // Update the checklist
                for line in &mut lines {
                    if line.contains(&format!("Step {}:", completed_step)) {
                        *line = line.replace("- [ ]", "- [x]");
                    }
                }

                // Update progress section
                let total_steps = lines.iter().filter(|line| line.contains("Step ")).count();
                let completed_steps = lines.iter().filter(|line| line.contains("- [x]")).count();

                for line in &mut lines {
                    if line.contains("Total Steps:") {
                        *line = format!("- Total Steps: {}", total_steps);
                    } else if line.contains("Completed:") {
                        *line = format!("- Completed: {}", completed_steps);
                    } else if line.contains("Remaining:") {
                        *line = format!("- Remaining: {}", total_steps.saturating_sub(completed_steps));
                    }
                }

                let new_content = lines.join("\n");
                fs::write("plan.md", new_content)?;
                Ok(format!("Updated plan.md: Step {} completed. Progress: {}/{}", completed_step, completed_steps, total_steps))
            }

            Tool::ClearPlan => {
                if Path::new("plan.md").exists() {
                    fs::remove_file("plan.md")?;
                    Ok("Cleared plan.md - task completed!".to_string())
                } else {
                    Ok("No plan.md file found to clear".to_string())
                }
            }
        }
    }

    // Helper methods for code execution
    fn execute_python(code: &str) -> Result<String, io::Error> {
        let temp_file = format!("/tmp/temp_code_{}.py", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
        fs::write(&temp_file, code)?;
        let output = Command::new("python3").arg(&temp_file).output()?;
        let _ = fs::remove_file(temp_file);
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Ok(format!("Python execution failed:\n{}", String::from_utf8_lossy(&output.stderr)))
        }
    }

    fn execute_javascript(code: &str) -> Result<String, io::Error> {
        let temp_file = format!("/tmp/temp_code_{}.js", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
        fs::write(&temp_file, code)?;
        let output = Command::new("node").arg(&temp_file).output()?;
        let _ = fs::remove_file(temp_file);
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Ok(format!("JavaScript execution failed:\n{}", String::from_utf8_lossy(&output.stderr)))
        }
    }

    fn execute_bash(code: &str) -> Result<String, io::Error> {
        let output = Command::new("bash").arg("-c").arg(code).output()?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Ok(format!("Bash execution failed:\n{}", String::from_utf8_lossy(&output.stderr)))
        }
    }

    fn execute_rust(code: &str) -> Result<String, io::Error> {
                        let temp_dir = format!("/tmp/rust_code_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
                        fs::create_dir_all(&temp_dir)?;
                        let main_rs = format!("{}/src/main.rs", temp_dir);
                        let cargo_toml = format!("{}/Cargo.toml", temp_dir);
                        
                        fs::write(&cargo_toml, r#"[package]
name = "temp_code"
version = "0.1.0"
edition = "2021"
[dependencies]
"#)?;
                        fs::create_dir_all(format!("{}/src", temp_dir))?;
                        fs::write(&main_rs, code)?;
        let mut cmd = Command::new("cargo");
        cmd.arg("run").current_dir(&temp_dir);
        let output = cmd.output()?;
        let _ = fs::remove_dir_all(temp_dir);
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Ok(format!("Rust execution failed:\n{}", String::from_utf8_lossy(&output.stderr)))
        }
    }

    fn execute_go(code: &str) -> Result<String, io::Error> {
        let temp_file = format!("/tmp/temp_code_{}.go", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
        fs::write(&temp_file, format!("package main\n\n{}", code))?;
        let output = Command::new("go").arg("run").arg(&temp_file).output()?;
        let _ = fs::remove_file(temp_file);
                        if output.status.success() {
                            Ok(String::from_utf8_lossy(&output.stdout).to_string())
                        } else {
            Ok(format!("Go execution failed:\n{}", String::from_utf8_lossy(&output.stderr)))
        }
    }

    fn execute_java(code: &str) -> Result<String, io::Error> {
        let temp_file = format!("/tmp/temp_code_{}.java", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
        let class_name = "TempCode";
        let full_code = format!("public class {} {{\n    public static void main(String[] args) {{\n        {}\n    }}\n}}", class_name, code);
        fs::write(&temp_file, full_code)?;
        let compile_output = Command::new("javac").arg(&temp_file).output()?;
        if compile_output.status.success() {
            let run_output = Command::new("java").arg("-cp").arg("/tmp").arg(class_name).output()?;
            let _ = fs::remove_file(temp_file);
            let _ = fs::remove_file(format!("/tmp/{}.class", class_name));
            if run_output.status.success() {
                Ok(String::from_utf8_lossy(&run_output.stdout).to_string())
            } else {
                Ok(format!("Java execution failed:\n{}", String::from_utf8_lossy(&run_output.stderr)))
            }
        } else {
            let _ = fs::remove_file(temp_file);
            Ok(format!("Java compilation failed:\n{}", String::from_utf8_lossy(&compile_output.stderr)))
        }
    }

    fn execute_c_cpp(code: &str, language: &str) -> Result<String, io::Error> {
        let is_cpp = matches!(language.to_lowercase().as_str(), "cpp" | "c++");
        let extension = if is_cpp { "cpp" } else { "c" };
        let temp_source = format!("/tmp/temp_code_{}.{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), extension);
        let temp_exe = format!("/tmp/temp_exe_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());

        let full_code = if is_cpp {
            format!("#include <iostream>\nint main() {{\n{}\nreturn 0;\n}}", code)
        } else {
            format!("#include <stdio.h>\nint main() {{\n{}\nreturn 0;\n}}", code)
        };

        fs::write(&temp_source, full_code)?;
        let compiler = if is_cpp { "g++" } else { "gcc" };
        let compile_output = Command::new(compiler).arg(&temp_source).arg("-o").arg(&temp_exe).output()?;
        if compile_output.status.success() {
            let run_output = Command::new(&temp_exe).output()?;
            let _ = fs::remove_file(temp_source);
            let _ = fs::remove_file(temp_exe);
            if run_output.status.success() {
                Ok(String::from_utf8_lossy(&run_output.stdout).to_string())
            } else {
                Ok(format!("{} execution failed:\n{}", if is_cpp { "C++" } else { "C" }, String::from_utf8_lossy(&run_output.stderr)))
            }
        } else {
            let _ = fs::remove_file(temp_source);
            Ok(format!("{} compilation failed:\n{}", if is_cpp { "C++" } else { "C" }, String::from_utf8_lossy(&compile_output.stderr)))
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
        r#"You are an advanced AI coding assistant with comprehensive access to development tools. You excel at software development, debugging, and project management. You MUST use tools to complete tasks - never just describe what you would do.

**CRITICAL FIRST STEP: For complex tasks requiring 3+ steps or multiple components, ALWAYS start with CREATE_PLAN as your very first tool call. Do not execute any other tools until the plan is created.**

## AVAILABLE TOOLS

### Planning and Task Management
1. **CREATE_PLAN** `<task> <steps>` - Create a structured plan in plan.md breaking down tasks into steps
2. **UPDATE_PLAN** `<step_number>` - Mark a specific step as completed in plan.md
3. **CLEAR_PLAN** - Remove plan.md when task is fully completed

### File Operations
4. **READ_FILE** `<path>` - Read and display file contents with line numbers
5. **WRITE_FILE** `<path> <content>` - Create or overwrite files (creates parent directories automatically)
6. **APPEND_FILE** `<path> <content>` - Add content to the end of existing files
7. **SEARCH_REPLACE** `<path> <old_string> <new_string>` - Replace text in files with exact matching
8. **DELETE_FILE** `<path>` - Remove files or directories (recursive for directories)

### Directory Operations
9. **LIST_FILES** `<path>` - List files and directories in a given path
10. **LIST_FILES_RECURSIVE** `<path>` - Recursively list all files in directory tree
11. **CREATE_DIRECTORY** `<path>` - Create directories (recursive)

### Search & Navigation
12. **GREP_SEARCH** `<pattern> [path]` - Search for text patterns using ripgrep (fast, regex support)
13. **GLOB_SEARCH** `<pattern>` - Find files matching glob patterns (*.rs, **/test/**, etc.)

### Code Execution & Compilation
14. **EXECUTE_CODE** `<language> <code>` - Execute code in multiple languages:
    - Python (python, py)
    - JavaScript/Node.js (javascript, js, node)
    - Bash/Shell (bash, sh)
    - Rust (rust)
    - Go (go)
    - Java (java)
    - C/C++ (c, cpp, c++)
15. **RUN_COMMAND** `<command>` - Execute shell commands with full environment access

### Development Workflow
16. **GIT_STATUS** - Show git repository status
17. **GIT_DIFF** - Show unstaged changes
18. **GIT_COMMIT** `<message>` - Commit changes with message
19. **GIT_LOG** `[count]` - Show recent commit history

### Quality Assurance
20. **RUN_LINT** `<language> [path]` - Run linters for code quality:
    - Rust: cargo clippy
    - Python: flake8/pylint
    - JavaScript: eslint
    - Go: golangci-lint
21. **RUN_TESTS** `<framework> [path]` - Run test suites:
    - Rust: cargo test
    - Python: pytest/unittest
    - JavaScript: jest/mocha
    - Go: go test

### Package Management
22. **INSTALL_PACKAGE** `<manager> <package>` - Install packages:
    - npm install <package>
    - cargo add <package>
    - pip install <package>
    - go get <package>

## PLANNING WORKFLOW - REQUIRED FOR COMPLEX TASKS

**MANDATORY PLANNING REQUIREMENT:**

**ALWAYS USE CREATE_PLAN for tasks that require 3+ steps or involve multiple files/components.**

**Examples of tasks that REQUIRE planning:**
- Building a web application (frontend + backend)
- Creating a new project from scratch
- Implementing authentication systems
- Setting up CI/CD pipelines
- Refactoring large codebases
- Adding new features with multiple components
- Database design and implementation
- API development with multiple endpoints

**Examples of tasks that DON'T require planning:**
- Reading a single file
- Simple text search
- Running a single command
- Making a small code change in one file

**PLANNING PROCESS - ALWAYS FOLLOW THIS EXACT SEQUENCE:**

1. **FIRST RESPONSE**: Always start with CREATE_PLAN for complex tasks
   ```
   TOOL: {"name": "CREATE_PLAN", "parameters": {"task": "Build a React todo app", "steps": ["Set up React project", "Create components", "Implement state management", "Add styling", "Test functionality"]}}
   ```

2. **EXECUTE STEPS**: Complete each step one by one
   ```
   TOOL: {"name": "UPDATE_PLAN", "parameters": {"completed_step": 1}}
   ```

3. **FINAL STEP**: Clean up when done
   ```
   TOOL: {"name": "CLEAR_PLAN", "parameters": {}}
   ```

**DECISION TREE FOR TASK HANDLING:**

```
User Request Received
├── Is it a simple task? (1-2 steps, single file)
│   ├── YES → Use tools directly (READ_FILE, WRITE_FILE, etc.)
│   └── NO → Continue to planning
└── Is it a complex task? (3+ steps, multiple files/components)
    ├── YES → CREATE_PLAN as FIRST tool call
    └── NO → Use tools directly

EXAMPLES:
✅ Simple: "Read the main.rs file" → READ_FILE
✅ Simple: "Change function name" → SEARCH_REPLACE
❌ Complex: "Build a web app" → CREATE_PLAN first
❌ Complex: "Add authentication" → CREATE_PLAN first
```

**CRITICAL: If you receive a complex task, your FIRST tool call MUST be CREATE_PLAN. Do not execute any other tools until the plan is created.**

## TOOL CALLING FORMAT

**PRIMARY FORMAT: Use JSON tool calls for maximum reliability:**
```json
TOOL: {"name": "READ_FILE", "parameters": {"path": "/path/to/file.txt"}}
TOOL: {"name": "CREATE_PLAN", "parameters": {"task": "Task description", "steps": ["Step 1", "Step 2", "Step 3"]}}
```

**Legacy format (still supported):**
```
TOOL: READ_FILE /path/to/file.txt
TOOL: CREATE_PLAN "Task description" "Step 1" "Step 2" "Step 3"
```

**CRITICAL: You must output the tool call EXACTLY as shown above. Do NOT describe what you would do - actually call the tools!**

## CODING WORKFLOW PRINCIPLES

### 1. Task Assessment Phase
- **Determine task complexity**: If task requires 3+ steps or multiple files → USE CREATE_PLAN
- **Assess scope**: Single file changes = direct tools, multi-component tasks = planning required

### 2. Exploration Phase
- Always start with LIST_FILES to understand project structure
- Use GLOB_SEARCH to find relevant files (*.rs for Rust, *.py for Python, etc.)
- READ_FILE key configuration files (Cargo.toml, package.json, requirements.txt, etc.)

### 3. Planning Phase (MANDATORY for complex tasks)
- **FIRST STEP**: Use CREATE_PLAN to break down the task
- Break complex tasks into manageable steps
- Identify dependencies and prerequisites
- Plan file modifications before executing

### 4. Implementation Phase
- Use SEARCH_REPLACE for precise edits (prefer over WRITE_FILE for modifications)
- APPEND_FILE for adding to existing files
- Verify changes with READ_FILE
- Test modifications with EXECUTE_CODE or RUN_COMMAND

### 5. Verification Phase
- Use RUN_LINT to check code quality
- Execute tests with RUN_TESTS
- Build/compile with RUN_COMMAND
- Verify functionality with EXECUTE_CODE

### 6. Completion Phase
- Use UPDATE_PLAN to mark steps as completed
- Use CLEAR_PLAN when all steps are done

### 7. Error Recovery
- If SEARCH_REPLACE fails, check exact string matching
- If EXECUTE_CODE fails, try RUN_COMMAND with compilation
- If RUN_COMMAND fails, simplify the command or check permissions
- Always verify file existence with LIST_FILES before operations

## EXAMPLES

### Planning:
TOOL: {"name": "CREATE_PLAN", "parameters": {"task": "Build a React component", "steps": ["Create component file", "Add state management", "Implement event handlers", "Add styling", "Test component"]}}
TOOL: {"name": "UPDATE_PLAN", "parameters": {"completed_step": 1}}
TOOL: {"name": "CLEAR_PLAN", "parameters": {}}

### File Operations:
TOOL: {"name": "READ_FILE", "parameters": {"path": "src/main.rs"}}
TOOL: {"name": "WRITE_FILE", "parameters": {"path": "hello.txt", "content": "Hello World!"}}
TOOL: {"name": "SEARCH_REPLACE", "parameters": {"path": "src/main.rs", "old_string": "fn main() {", "new_string": "fn main() {\n    println!(\"Hello!\");"}}

### Development Tasks:
TOOL: {"name": "LIST_FILES", "parameters": {"path": "."}}
TOOL: {"name": "GREP_SEARCH", "parameters": {"pattern": "TODO|FIXME", "path": "src/"}}
TOOL: {"name": "EXECUTE_CODE", "parameters": {"language": "rust", "code": "fn main() { println!(\"test\"); }"}}
TOOL: {"name": "RUN_LINT", "parameters": {"language": "rust"}}

### Quality Assurance:
TOOL: {"name": "RUN_TESTS", "parameters": {"framework": "cargo"}}
TOOL: {"name": "GIT_STATUS", "parameters": {}}
TOOL: {"name": "RUN_COMMAND", "parameters": {"command": "cargo build --release"}}

## CRITICAL GUIDELINES

- **PLAN FIRST for complex tasks** - Always use CREATE_PLAN as your FIRST tool call for tasks requiring 3+ steps
- **ALWAYS use tools** - Never describe actions without executing them
- **Use JSON format** - Prefer `TOOL: {"name": "TOOL_NAME", "parameters": {...}}` over legacy format
- **Execute immediately** - Do not explain what you would do, just call the tool
- **Verify work** - Read files after writing, list directories after changes
- **Handle errors gracefully** - Try alternative approaches when tools fail
- **Be precise** - Use exact string matching for SEARCH_REPLACE
- **Think systematically** - Plan before acting, verify after completion
- **Code quality matters** - Use linters and tests to maintain standards

**REMEMBER: For complex tasks, your FIRST response MUST contain CREATE_PLAN. For simple tasks, use tools directly. Your responses should contain actual tool calls that will be executed, not descriptions of tool usage.**"#.to_string()
    }

    fn parse_tool_call(&self, response: &str) -> Option<Tool> {
        let lines: Vec<&str> = response.lines().collect();
        for line in lines {
            if line.starts_with("TOOL:") {
                let tool_part = line[6..].trim();

                // Try JSON format first
                if tool_part.starts_with('{') {
                    if let Ok(tool_call) = serde_json::from_str::<ToolCall>(tool_part) {
                        if let Some(tool) = tool_call.into_tool() {
                            return Some(tool);
                        }
                    }
                }

                // Fall back to legacy format
                let parts: Vec<&str> = tool_part.splitn(2, ' ').collect();
                if parts.len() >= 2 {
                    let tool_name = parts[0];
                    let params = parts[1];
                    
                    return match tool_name {
                        "READ_FILE" => Some(Tool::ReadFile { path: params.to_string() }),
                        "WRITE_FILE" => {
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
                        "APPEND_FILE" => {
                            if let Some(space_pos) = params.find(' ') {
                                let path = &params[..space_pos];
                                let content = &params[space_pos + 1..];
                                Some(Tool::AppendFile {
                                    path: path.to_string(),
                                    content: content.to_string()
                                })
                            } else {
                                None
                            }
                        }
                        "SEARCH_REPLACE" => {
                            let params_str = params.to_string();
                            // Parse: "path old_string new_string" - need to split carefully
                            if let Some(first_space) = params_str.find(' ') {
                                let path = &params_str[..first_space];
                                let remaining = &params_str[first_space + 1..];
                                // Find the boundary between old_string and new_string
                                // This is tricky with the legacy format, but we'll assume old_string comes first
                                if let Some(last_space) = remaining.rfind(' ') {
                                    let old_string = &remaining[..last_space];
                                    let new_string = &remaining[last_space + 1..];
                                    Some(Tool::SearchReplace {
                                        path: path.to_string(),
                                        old_string: old_string.to_string(),
                                        new_string: new_string.to_string(),
                                    })
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        "RUN_COMMAND" => Some(Tool::RunCommand { command: params.to_string() }),
                        "LIST_FILES" => Some(Tool::ListFiles { path: params.to_string() }),
                        "LIST_FILES_RECURSIVE" => Some(Tool::ListFilesRecursive { path: params.to_string() }),
                        "CREATE_DIRECTORY" => Some(Tool::CreateDirectory { path: params.to_string() }),
                        "DELETE_FILE" => Some(Tool::DeleteFile { path: params.to_string() }),
                        "EXECUTE_CODE" => {
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
                        "GREP_SEARCH" => {
                            // Parse: "pattern [path]"
                            let search_parts: Vec<&str> = params.splitn(2, ' ').collect();
                            let pattern = search_parts[0].to_string();
                            let path = search_parts.get(1).map(|s| s.to_string());
                            Some(Tool::GrepSearch { pattern, path })
                        }
                        "GLOB_SEARCH" => Some(Tool::GlobSearch { pattern: params.to_string() }),
                        "GIT_STATUS" => Some(Tool::GitStatus),
                        "GIT_DIFF" => Some(Tool::GitDiff),
                        "GIT_COMMIT" => Some(Tool::GitCommit { message: params.to_string() }),
                        "GIT_LOG" => {
                            let count = if params.is_empty() {
                                None
                            } else {
                                params.parse::<usize>().ok()
                            };
                            Some(Tool::GitLog { count })
                        }
                        "RUN_LINT" => {
                            let lint_parts: Vec<&str> = params.splitn(2, ' ').collect();
                            let language = lint_parts[0].to_string();
                            let path = lint_parts.get(1).map(|s| s.to_string());
                            Some(Tool::RunLint { language, path })
                        }
                        "RUN_TESTS" => {
                            let test_parts: Vec<&str> = params.splitn(2, ' ').collect();
                            let framework = test_parts[0].to_string();
                            let path = test_parts.get(1).map(|s| s.to_string());
                            Some(Tool::RunTests { framework, path })
                        }
                        "INSTALL_PACKAGE" => {
                            let package_parts: Vec<&str> = params.splitn(2, ' ').collect();
                            if package_parts.len() == 2 {
                                Some(Tool::InstallPackage {
                                    manager: package_parts[0].to_string(),
                                    package: package_parts[1].to_string(),
                                })
                            } else {
                                None
                            }
                        }
                        "CREATE_PLAN" => {
                            // Parse: "task description" "step1" "step2" "step3"
                            let parts: Vec<&str> = params.split('"').filter(|s| !s.is_empty() && !s.trim().is_empty()).collect();
                            if parts.len() >= 2 {
                                let task = parts[0].trim().to_string();
                                let steps: Vec<String> = parts[1..].iter().map(|s| s.trim().to_string()).collect();
                                Some(Tool::CreatePlan { task, steps })
                            } else {
                                None
                            }
                        }
                        "UPDATE_PLAN" => {
                            if let Ok(step) = params.trim().parse::<usize>() {
                                Some(Tool::UpdatePlan { completed_step: step })
                            } else {
                                None
                            }
                        }
                        "CLEAR_PLAN" => Some(Tool::ClearPlan),
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

        // Add planning reminder for complex tasks
        if self.messages.len() == 1 { // Only add once, after system message
            self.messages.push(Message {
                role: "system".to_string(),
                content: "REMINDER: If the user's request is complex (3+ steps, multiple files), start with CREATE_PLAN. For simple tasks, use tools directly.".to_string(),
            });
        }

        // Add task complexity analysis
        let user_input_lower = user_prompt.to_lowercase();
        let complex_keywords = ["create", "build", "implement", "develop", "setup", "add", "design", "refactor", "migrate"];
        let has_complex_keywords = complex_keywords.iter().any(|&keyword| user_input_lower.contains(keyword));
        let is_complex = has_complex_keywords ||
                        user_input_lower.contains("app") ||
                        user_input_lower.contains("application") ||
                        user_input_lower.contains("system") ||
                        user_input_lower.contains("api") ||
                        user_input_lower.contains("server") ||
                        user_input_lower.contains("database");

        if is_complex && self.messages.len() == 2 { // Only for first user message
            self.messages.push(Message {
                role: "system".to_string(),
                content: "This appears to be a complex task. Please start with CREATE_PLAN to break it down into manageable steps.".to_string(),
            });
        }

        // Add user message
        self.messages.push(Message {
            role: "user".to_string(),
            content: user_prompt.clone(),
        });

        let mut all_tool_logs = Vec::new();
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 8; // Increased for better error recovery

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
                    Tool::AppendFile { path, content: _ } => format!("APPEND_FILE {}", path),
                    Tool::SearchReplace { path, old_string: _, new_string: _ } => format!("SEARCH_REPLACE {}", path),
                    Tool::DeleteFile { path } => format!("DELETE_FILE {}", path),
                    Tool::ListFiles { path } => format!("LIST_FILES {}", path),
                    Tool::ListFilesRecursive { path } => format!("LIST_FILES_RECURSIVE {}", path),
                    Tool::CreateDirectory { path } => format!("CREATE_DIRECTORY {}", path),
                    Tool::GrepSearch { pattern, path: _ } => format!("GREP_SEARCH {}", pattern),
                    Tool::GlobSearch { pattern } => format!("GLOB_SEARCH {}", pattern),
                    Tool::ExecuteCode { language, code: _ } => format!("EXECUTE_CODE {}", language),
                    Tool::RunCommand { command } => format!("RUN_COMMAND {}", command),
                    Tool::GitStatus => "GIT_STATUS".to_string(),
                    Tool::GitDiff => "GIT_DIFF".to_string(),
                    Tool::GitCommit { message } => format!("GIT_COMMIT \"{}\"", message),
                    Tool::GitLog { count } => format!("GIT_LOG {}", count.map_or("all".to_string(), |n| n.to_string())),
                    Tool::RunLint { language, path: _ } => format!("RUN_LINT {}", language),
                    Tool::RunTests { framework, path: _ } => format!("RUN_TESTS {}", framework),
                    Tool::InstallPackage { manager, package } => format!("INSTALL_PACKAGE {} {}", manager, package),
                    Tool::CreatePlan { task, steps } => format!("CREATE_PLAN \"{}\" ({} steps)", task, steps.len()),
                    Tool::UpdatePlan { completed_step } => format!("UPDATE_PLAN step {}", completed_step),
                    Tool::ClearPlan => "CLEAR_PLAN".to_string(),
                };
                tool_logs.push(format!("🔧 Attempt {}: Executing {}", attempts, tool_name));
                
                // Execute the tool
                let tool_result = match tool.execute() {
                    Ok(result) => {
                        tool_logs.push(format!("✅ Success: {}", result));
                        result
                    }
                    Err(e) => {
                        let error_msg = format!("❌ Error: {}", e);
                        tool_logs.push(error_msg.clone());
                        // Add error context to help the model understand what went wrong
                        format!("Tool failed: {}. Please try a different approach or check if the path/command is correct.", e)
                    }
                };
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
