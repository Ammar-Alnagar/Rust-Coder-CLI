use rust_tui_coder::agent::Tool;
use rust_tui_coder::app::App;
use std::fs;

#[test]
fn test_integration_file_workflow() {
    let test_dir = "tmp_rovodev_integration_test";
    let test_file = format!("{}/test.txt", test_dir);
    
    // Create directory
    let create_dir = Tool::CreateDirectory {
        path: test_dir.to_string(),
    };
    assert!(create_dir.execute().is_ok());
    
    // Write file
    let write_file = Tool::WriteFile {
        path: test_file.clone(),
        content: "Initial content".to_string(),
    };
    assert!(write_file.execute().is_ok());
    
    // Read file
    let read_file = Tool::ReadFile {
        path: test_file.clone(),
    };
    let result = read_file.execute();
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Initial content"));
    
    // Append to file
    let append_file = Tool::AppendFile {
        path: test_file.clone(),
        content: "\nAppended content".to_string(),
    };
    assert!(append_file.execute().is_ok());
    
    // Read again
    let read_file2 = Tool::ReadFile {
        path: test_file.clone(),
    };
    let result2 = read_file2.execute();
    assert!(result2.is_ok());
    let content = result2.unwrap();
    assert!(content.contains("Initial content"));
    assert!(content.contains("Appended content"));
    
    // Search and replace
    let search_replace = Tool::SearchReplace {
        path: test_file.clone(),
        old_string: "Initial".to_string(),
        new_string: "Modified".to_string(),
    };
    assert!(search_replace.execute().is_ok());
    
    // Verify replacement
    let read_file3 = Tool::ReadFile {
        path: test_file.clone(),
    };
    let result3 = read_file3.execute();
    assert!(result3.is_ok());
    assert!(result3.unwrap().contains("Modified content"));
    
    // Clean up
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_integration_app_workflow() {
    let mut app = App::new();
    
    // Simulate adding messages
    app.conversation.push("User: Hello".to_string());
    app.conversation.push("Agent: Hi there!".to_string());
    assert_eq!(app.conversation.len(), 2);
    
    // Add tool logs
    app.add_tool_log("Executed READ_FILE".to_string());
    app.add_tool_log("Executed WRITE_FILE".to_string());
    assert_eq!(app.tool_logs.len(), 2);
    
    // Update usage stats
    app.increment_tokens(500);
    app.increment_requests();
    app.increment_tools_executed();
    app.increment_tools_executed();
    
    assert_eq!(app.tokens_used, 500);
    assert_eq!(app.total_requests, 1);
    assert_eq!(app.total_tools_executed, 2);
    
    // Test scrolling
    app.scroll_conversation_down();
    app.scroll_conversation_down();
    assert_eq!(app.conversation_scroll_position, 2);
    
    app.scroll_conversation_up();
    assert_eq!(app.conversation_scroll_position, 1);
    
    app.scroll_conversation_to_bottom();
    assert_eq!(app.conversation_scroll_position, usize::MAX);
    
    app.scroll_conversation_to_top();
    assert_eq!(app.conversation_scroll_position, 0);
}

#[test]
fn test_integration_plan_workflow() {
    // Create a plan
    let create_plan = Tool::CreatePlan {
        task: "Integration Test Task".to_string(),
        steps: vec![
            "Step 1: Setup".to_string(),
            "Step 2: Execute".to_string(),
            "Step 3: Verify".to_string(),
        ],
    };
    assert!(create_plan.execute().is_ok());
    
    // Update plan for each step
    let update_step1 = Tool::UpdatePlan { completed_step: 1 };
    assert!(update_step1.execute().is_ok());
    
    let update_step2 = Tool::UpdatePlan { completed_step: 2 };
    assert!(update_step2.execute().is_ok());
    
    let update_step3 = Tool::UpdatePlan { completed_step: 3 };
    assert!(update_step3.execute().is_ok());
    
    // Verify all steps are completed
    let content = fs::read_to_string("plan.md").unwrap();
    assert!(content.contains("Completed: 3"));
    
    // Clear the plan
    let clear_plan = Tool::ClearPlan;
    assert!(clear_plan.execute().is_ok());
    assert!(!std::path::Path::new("plan.md").exists());
}

#[test]
fn test_integration_streaming_workflow() {
    let mut app = App::new();
    
    // Start streaming
    app.start_streaming();
    assert!(app.is_streaming);
    assert_eq!(app.current_streaming_message, "");
    
    // Simulate streaming chunks
    app.update_streaming_message("Hello");
    app.update_streaming_message(" ");
    app.update_streaming_message("World");
    app.update_streaming_message("!");
    
    assert_eq!(app.current_streaming_message, "Hello World!");
    
    // Finish streaming
    app.finish_streaming("Hello World!".to_string());
    assert!(!app.is_streaming);
    assert_eq!(app.current_streaming_message, "");
    assert_eq!(app.conversation.len(), 1);
    assert_eq!(app.conversation[0], "Agent: Hello World!");
    assert_eq!(app.conversation_scroll_position, usize::MAX);
}

#[test]
fn test_integration_multiple_file_operations() {
    let test_dir = "tmp_rovodev_multi_ops";
    fs::create_dir_all(test_dir).ok();
    
    // Create multiple files
    for i in 1..=5 {
        let write_tool = Tool::WriteFile {
            path: format!("{}/file{}.txt", test_dir, i),
            content: format!("Content {}", i),
        };
        assert!(write_tool.execute().is_ok());
    }
    
    // List files
    let list_tool = Tool::ListFiles {
        path: test_dir.to_string(),
    };
    let result = list_tool.execute();
    assert!(result.is_ok());
    let output = result.unwrap();
    
    for i in 1..=5 {
        assert!(output.contains(&format!("file{}.txt", i)));
    }
    
    // Clean up
    fs::remove_dir_all(test_dir).ok();
}
