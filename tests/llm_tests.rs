use rust_tui_coder::llm::{estimate_token_count, Message};

#[test]
fn test_estimate_token_count_empty() {
    assert_eq!(estimate_token_count(""), 0);
}

#[test]
fn test_estimate_token_count_short() {
    let count = estimate_token_count("Hello");
    assert!(count >= 1);
}

#[test]
fn test_estimate_token_count_long() {
    let text = "This is a longer text that should be tokenized into multiple tokens based on the estimation algorithm.";
    let count = estimate_token_count(text);
    assert!(count > 10);
}

#[test]
fn test_estimate_token_count_whitespace() {
    let count = estimate_token_count("   ");
    assert!(count >= 0);
}

#[test]
fn test_estimate_token_count_unicode() {
    let count = estimate_token_count("Hello 世界 🌍");
    assert!(count >= 1);
}

#[test]
fn test_message_creation() {
    let message = Message {
        role: "user".to_string(),
        content: "Hello, AI!".to_string(),
    };
    
    assert_eq!(message.role, "user");
    assert_eq!(message.content, "Hello, AI!");
}

#[test]
fn test_message_clone() {
    let message = Message {
        role: "assistant".to_string(),
        content: "Hello, human!".to_string(),
    };
    
    let cloned = message.clone();
    assert_eq!(message.role, cloned.role);
    assert_eq!(message.content, cloned.content);
}

#[test]
fn test_message_serialization() {
    let message = Message {
        role: "system".to_string(),
        content: "You are a helpful assistant.".to_string(),
    };
    
    let json = serde_json::to_string(&message);
    assert!(json.is_ok());
    
    let json_str = json.unwrap();
    assert!(json_str.contains("system"));
    assert!(json_str.contains("You are a helpful assistant"));
}

#[test]
fn test_message_deserialization() {
    let json_str = r#"{"role":"user","content":"Test message"}"#;
    let result: Result<Message, _> = serde_json::from_str(json_str);
    
    assert!(result.is_ok());
    let message = result.unwrap();
    assert_eq!(message.role, "user");
    assert_eq!(message.content, "Test message");
}
