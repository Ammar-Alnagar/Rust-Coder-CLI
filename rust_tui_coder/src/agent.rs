use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::process::Command;
use crate::llm::{self, Message, LlmError};
use crate::config::LlmConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
        Self {
            messages: vec![],
        }
    }

    pub async fn run(&mut self, config: &LlmConfig, user_prompt: String) -> Result<String, Box<dyn std::error::Error>> {
        // For now, just a simple call to the LLM.
        // The logic for tool parsing will be added later.
        let response = llm::ask_llm(config, user_prompt).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        Ok(response)
    }
}
