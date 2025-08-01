use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::config::LlmConfig;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LlmError {
    RequestFailed(reqwest::Error),
    ApiError(String),
    ParseError(String),
}

impl fmt::Display for LlmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LlmError::RequestFailed(e) => write!(f, "Request failed: {}", e),
            LlmError::ApiError(msg) => write!(f, "API error: {}", msg),
            LlmError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl Error for LlmError {}

impl From<reqwest::Error> for LlmError {
    fn from(err: reqwest::Error) -> Self {
        LlmError::RequestFailed(err)
    }
}

#[derive(Serialize, Debug)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

pub async fn ask_llm(config: &LlmConfig, prompt: String) -> Result<String, LlmError> {
    let client = Client::new();

    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "You are a helpful assistant.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: prompt,
        },
    ];

    let request_body = ChatCompletionRequest {
        model: config.model_name.clone(),
        messages,
    };

    // Debug: Print request details
    eprintln!("Making request to: {}/chat/completions", config.api_base_url);
    eprintln!("Model: {}", config.model_name);
    eprintln!("API Key (first 4 chars): {}", &config.api_key[..4.min(config.api_key.len())]);

    // First, get the raw response to debug
    let response = client
        .post(&format!("{}/chat/completions", config.api_base_url))
        .bearer_auth(&config.api_key)
        .json(&request_body)
        .send()
        .await?;

    // Check if the request was successful
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await?;
        eprintln!("API Error: Status {} - {}", status, error_text);
        return Err(LlmError::ApiError(format!("HTTP {}: {}", status, error_text)));
    }

    // Get the raw response text for debugging
    let response_text = response.text().await?;
    eprintln!("Raw API Response: {}", response_text);

    // Try to parse the response
    match serde_json::from_str::<ChatCompletionResponse>(&response_text) {
        Ok(parsed_response) => {
            if let Some(choice) = parsed_response.choices.into_iter().next() {
                Ok(choice.message.content)
            } else {
                Ok("No response content available.".to_string())
            }
        }
        Err(e) => {
            eprintln!("Failed to parse response: {}", e);
            eprintln!("Response was: {}", response_text);
            Err(LlmError::ParseError(format!("Failed to parse API response: {}", e)))
        }
    }
}
