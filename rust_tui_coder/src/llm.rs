use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::config::LlmConfig;

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

pub async fn ask_llm(config: &LlmConfig, messages: Vec<Message>) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let request_body = ChatCompletionRequest {
        model: config.model_name.clone(),
        messages,
    };

    let response: ChatCompletionResponse = client
        .post(&format!("{}/chat/completions", config.api_base_url))
        .bearer_auth(&config.api_key)
        .json(&request_body)
        .send()
        .await?
        .json()
        .await?;

    if let Some(choice) = response.choices.into_iter().next() {
        Ok(choice.message.content)
    } else {
        Ok("".to_string())
    }
}
