use rust_tui_coder::app::App;

#[test]
fn test_scrolling_behavior() {
    let mut app = App::new();

    // Add some conversation messages
    for i in 0..20 {
        app.conversation.push(format!("User: Message {}", i));
        app.conversation.push(format!("Agent: Response {}", i));
    }

    // Test scrolling down
    app.scroll_conversation_down();
    assert_eq!(app.conversation_scroll_position, 1);

    app.scroll_conversation_down();
    assert_eq!(app.conversation_scroll_position, 2);

    // Test scrolling up
    app.scroll_conversation_up();
    assert_eq!(app.conversation_scroll_position, 1);

    // Test boundary at top
    app.scroll_conversation_to_top();
    assert_eq!(app.conversation_scroll_position, 0);
    app.scroll_conversation_up();
    assert_eq!(app.conversation_scroll_position, 0);

    // Test scrolling to bottom
    app.scroll_conversation_to_bottom();
    assert_eq!(app.conversation_scroll_position, usize::MAX);

    // Test that scrolling up from max works
    app.scroll_conversation_up();
    assert!(app.conversation_scroll_position < usize::MAX);
}

#[test]
fn test_scroll_position_clamping() {
    let mut app = App::new();

    // Set scroll position beyond reasonable bounds
    app.conversation_scroll_position = 1000;

    // The UI should clamp this when rendering
    // This test verifies the scroll methods work correctly
    app.scroll_conversation_to_top();
    assert_eq!(app.conversation_scroll_position, 0);
}

#[test]
fn test_conversation_display() {
    let mut app = App::new();

    // Add mixed conversation
    app.conversation.push("User: Hello".to_string());
    app.conversation.push("Agent: Hi there!".to_string());
    app.conversation.push("System: Session started".to_string());

    assert_eq!(app.conversation.len(), 3);
    assert!(app.conversation[0].starts_with("User:"));
    assert!(app.conversation[1].starts_with("Agent:"));
    assert!(app.conversation[2].starts_with("System:"));
}

#[test]
fn test_tool_logs_display() {
    let mut app = App::new();

    // Add tool logs
    app.add_tool_log("Executed READ_FILE: success".to_string());
    app.add_tool_log("Executed WRITE_FILE: success".to_string());
    app.add_tool_log("Executed RUN_COMMAND: output".to_string());

    assert_eq!(app.tool_logs.len(), 3);
}

#[test]
fn test_streaming_state_management() {
    let mut app = App::new();

    // Test streaming lifecycle
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

    // After finishing, scroll should be at bottom
    assert_eq!(app.conversation_scroll_position, usize::MAX);
}

#[test]
fn test_page_scrolling() {
    let mut app = App::new();

    // Simulate page down (10 lines)
    for _ in 0..10 {
        app.scroll_conversation_down();
    }
    assert_eq!(app.conversation_scroll_position, 10);

    // Simulate page up (10 lines)
    for _ in 0..5 {
        app.scroll_conversation_up();
    }
    assert_eq!(app.conversation_scroll_position, 5);
}

#[test]
fn test_status_message_updates() {
    let mut app = App::new();

    // Initial status
    assert!(app.status_message.contains("Commands:"));

    // Update status
    app.status_message = "Processing...".to_string();
    assert_eq!(app.status_message, "Processing...");

    // Streaming status
    app.start_streaming();
    assert!(app.status_message.contains("streaming"));
}

#[test]
fn test_empty_conversation_scrolling() {
    let mut app = App::new();

    // Empty conversation should handle scrolling gracefully
    app.scroll_conversation_down();
    app.scroll_conversation_up();
    app.scroll_conversation_to_bottom();
    app.scroll_conversation_to_top();

    // No panics should occur
}

#[test]
fn test_tool_execution_state() {
    let mut app = App::new();

    // Initially not executing
    assert!(!app.is_executing_tool);
    assert_eq!(app.current_tool, "");

    // Simulate tool execution
    app.is_executing_tool = true;
    app.current_tool = "READ_FILE".to_string();

    assert!(app.is_executing_tool);
    assert_eq!(app.current_tool, "READ_FILE");

    // Complete execution
    app.is_executing_tool = false;
    app.current_tool = String::new();

    assert!(!app.is_executing_tool);
}
