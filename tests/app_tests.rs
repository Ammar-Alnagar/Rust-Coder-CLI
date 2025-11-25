use rust_tui_coder::app::App;

#[test]
fn test_app_new() {
    let app = App::new();
    assert_eq!(app.user_input, "");
    assert!(app.conversation.is_empty());
    assert_eq!(app.tokens_used, 0);
    assert_eq!(app.total_requests, 0);
    assert_eq!(app.total_tools_executed, 0);
    assert_eq!(app.conversation_scroll_position, 0);
    assert_eq!(app.tool_logs_scroll_position, 0);
    assert!(!app.is_streaming);
    assert_eq!(app.current_streaming_message, "");
}

#[test]
fn test_app_default() {
    let app = App::default();
    assert_eq!(app.user_input, "");
    assert!(app.conversation.is_empty());
}

#[test]
fn test_add_tool_log() {
    let mut app = App::new();
    app.add_tool_log("Test log 1".to_string());
    app.add_tool_log("Test log 2".to_string());
    assert_eq!(app.tool_logs.len(), 2);
    assert_eq!(app.tool_logs[0], "Test log 1");
    assert_eq!(app.tool_logs[1], "Test log 2");
}

#[test]
fn test_increment_tokens() {
    let mut app = App::new();
    app.increment_tokens(100);
    assert_eq!(app.tokens_used, 100);
    app.increment_tokens(50);
    assert_eq!(app.tokens_used, 150);
}

#[test]
fn test_increment_requests() {
    let mut app = App::new();
    app.increment_requests();
    assert_eq!(app.total_requests, 1);
    app.increment_requests();
    assert_eq!(app.total_requests, 2);
}

#[test]
fn test_increment_tools_executed() {
    let mut app = App::new();
    app.increment_tools_executed();
    assert_eq!(app.total_tools_executed, 1);
    app.increment_tools_executed();
    assert_eq!(app.total_tools_executed, 2);
}

#[test]
fn test_scroll_conversation_up() {
    let mut app = App::new();
    app.conversation_scroll_position = 10;
    app.scroll_conversation_up();
    assert_eq!(app.conversation_scroll_position, 9);
    
    // Test boundary condition
    app.conversation_scroll_position = 0;
    app.scroll_conversation_up();
    assert_eq!(app.conversation_scroll_position, 0);
}

#[test]
fn test_scroll_conversation_down() {
    let mut app = App::new();
    app.scroll_conversation_down();
    assert_eq!(app.conversation_scroll_position, 1);
    app.scroll_conversation_down();
    assert_eq!(app.conversation_scroll_position, 2);
}

#[test]
fn test_scroll_conversation_to_top() {
    let mut app = App::new();
    app.conversation_scroll_position = 100;
    app.scroll_conversation_to_top();
    assert_eq!(app.conversation_scroll_position, 0);
}

#[test]
fn test_scroll_conversation_to_bottom() {
    let mut app = App::new();
    app.scroll_conversation_to_bottom();
    assert_eq!(app.conversation_scroll_position, usize::MAX);
}

#[test]
fn test_streaming_state() {
    let mut app = App::new();
    assert!(!app.is_streaming);
    
    app.start_streaming();
    assert!(app.is_streaming);
    assert_eq!(app.current_streaming_message, "");
    
    app.update_streaming_message("Hello");
    assert_eq!(app.current_streaming_message, "Hello");
    
    app.update_streaming_message(" World");
    assert_eq!(app.current_streaming_message, "Hello World");
    
    app.finish_streaming("Hello World".to_string());
    assert!(!app.is_streaming);
    assert_eq!(app.current_streaming_message, "");
    assert_eq!(app.conversation.len(), 1);
    assert_eq!(app.conversation[0], "Agent: Hello World");
}

#[test]
fn test_usage_summary() {
    let mut app = App::new();
    app.increment_tokens(1000);
    app.increment_requests();
    app.increment_requests();
    app.increment_tools_executed();
    
    let summary = app.get_usage_summary();
    assert!(summary.contains("Tokens Used: 1000"));
    assert!(summary.contains("LLM Requests: 2"));
    assert!(summary.contains("Tools Executed: 1"));
    assert!(summary.contains("Average Tokens/Request: 500"));
}

#[test]
fn test_session_duration() {
    let app = App::new();
    std::thread::sleep(std::time::Duration::from_millis(100));
    let duration = app.get_session_duration();
    assert!(duration.as_millis() >= 100);
}
