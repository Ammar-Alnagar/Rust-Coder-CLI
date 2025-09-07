use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::config::LlmConfig;
use std::error::Error;
use std::fmt;

// Token estimation function (rough approximation based on GPT tokenization)
pub fn estimate_token_count(text: &str) -> u64 {
    // Rough approximation: ~4 characters per token for English text
    // This is not exact but provides a reasonable estimate
    let char_count = text.chars().count() as f64;
    let estimated_tokens = (char_count / 4.0).ceil() as u64;

    // Ensure minimum of 1 token for non-empty text
    if estimated_tokens == 0 && !text.trim().is_empty() {
        1
    } else {
        estimated_tokens
    }
}

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

#[derive(Deserialize, Debug)]
struct ModelsResponse {
    data: Vec<ModelInfo>,
}

#[derive(Deserialize, Debug)]
struct ModelInfo {
    id: String,
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

pub async fn ask_llm_with_messages(config: &LlmConfig, messages: &[Message]) -> Result<(String, u64), LlmError> {
    let client = Client::new();
    let _provider = config.provider.as_deref().unwrap_or("openai");

    // Calculate input tokens
    let mut input_tokens = 0u64;
    for message in messages {
        input_tokens += estimate_token_count(&message.role);
        input_tokens += estimate_token_count(&message.content);
    }

    // Determine model to use (support AUTODETECT from /models endpoint for OpenAI-compatible APIs)
    let model_to_use = if config.model_name.eq_ignore_ascii_case("AUTODETECT") || config.model_name.trim().is_empty() {
        // Fetch available models and use the first one
        let models_url = format!(
            "{}/models",
            config.api_base_url.trim_end_matches('/')
        );

        let mut models_request = client.get(&models_url);
        if !config.api_key.is_empty() {
            models_request = models_request.bearer_auth(&config.api_key);
        }
        let response = models_request.send().await?;

        let status = response.status();
        let response_text = response.text().await?;
        if !status.is_success() {
            return Err(LlmError::ApiError(format!(
                "HTTP {} while fetching models: {}",
                status, response_text
            )));
        }

        let parsed_models: ModelsResponse = serde_json::from_str(&response_text)
            .map_err(|e| LlmError::ParseError(format!(
                "Failed to parse /models response: {}",
                e
            )))?;

        match parsed_models.data.first() {
            Some(first) => first.id.clone(),
            None => return Err(LlmError::ApiError("No models returned by /models".to_string())),
        }
    } else {
        config.model_name.clone()
    };

    let request_body = ChatCompletionRequest {
        model: model_to_use,
        messages: messages.to_vec(),
    };

    // First, get the raw response to debug
    let mut completion_request = client
        .post(&format!(
            "{}/chat/completions",
            config.api_base_url.trim_end_matches('/')
        ))
        .json(&request_body);
    if !config.api_key.is_empty() {
        completion_request = completion_request.bearer_auth(&config.api_key);
    }
    let response = completion_request.send().await?;

    // Check if the request was successful
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(LlmError::ApiError(format!("HTTP {}: {}", status, error_text)));
    }

    // Get the raw response text for debugging
    let response_text = response.text().await?;

    // Try to parse the response
    match serde_json::from_str::<ChatCompletionResponse>(&response_text) {
        Ok(parsed_response) => {
            if let Some(choice) = parsed_response.choices.into_iter().next() {
                let response_content = choice.message.content;
                let output_tokens = estimate_token_count(&response_content);
                let total_tokens = input_tokens + output_tokens;
                Ok((response_content, total_tokens))
            } else {
                let response_content = "No response content available.".to_string();
                let output_tokens = estimate_token_count(&response_content);
                let total_tokens = input_tokens + output_tokens;
                Ok((response_content, total_tokens))
            }
        }
        Err(e) => {
            Err(LlmError::ParseError(format!("Failed to parse API response: {}", e)))
        }
    }
}
