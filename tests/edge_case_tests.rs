use rust_tui_coder::agent::Tool;
use rust_tui_coder::app::App;
use rust_tui_coder::llm::estimate_token_count;
use std::fs;

#[test]
fn test_empty_file_operations() {
    let test_file = "tmp_rovodev_empty.txt";

    // Write empty file
    let write = Tool::WriteFile {
        path: test_file.to_string(),
        content: String::new(),
    };
    assert!(write
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    // Read empty file
    let read = Tool::ReadFile {
        path: test_file.to_string(),
    };
    let result = read.execute(&rust_tui_coder::config::WebConfig::default());
    assert!(result.is_ok());

    // Append to empty file
    let append = Tool::AppendFile {
        path: test_file.to_string(),
        content: "Added content".to_string(),
    };
    assert!(append
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    fs::remove_file(test_file).ok();
}

#[test]
fn test_special_characters_in_content() {
    let test_file = "tmp_rovodev_special_chars.txt";

    let special_content =
        "Hello\nWorld\t!\r\n\"Quotes\" and 'apostrophes'\n$pecial #characters @here";

    let write = Tool::WriteFile {
        path: test_file.to_string(),
        content: special_content.to_string(),
    };
    assert!(write
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    let read = Tool::ReadFile {
        path: test_file.to_string(),
    };
    let result = read.execute(&rust_tui_coder::config::WebConfig::default());
    assert!(result.is_ok());
    assert!(result.unwrap().contains("$pecial"));

    fs::remove_file(test_file).ok();
}

#[test]
fn test_unicode_content() {
    let test_file = "tmp_rovodev_unicode.txt";

    let unicode_content = "Hello 世界 🌍 🦀 Rust\nΔημοκρατία\nԱզգություն";

    let write = Tool::WriteFile {
        path: test_file.to_string(),
        content: unicode_content.to_string(),
    };
    assert!(write
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    let read = Tool::ReadFile {
        path: test_file.to_string(),
    };
    let result = read.execute(&rust_tui_coder::config::WebConfig::default());
    assert!(result.is_ok());
    let content = result.unwrap();
    assert!(content.contains("世界"));
    assert!(content.contains("🦀"));

    fs::remove_file(test_file).ok();
}

#[test]
fn test_very_long_filenames() {
    let test_dir = "tmp_rovodev_long_name";
    fs::create_dir_all(test_dir).ok();

    let long_filename = format!("{}/{}.txt", test_dir, "a".repeat(200));

    let write = Tool::WriteFile {
        path: long_filename.clone(),
        content: "Content".to_string(),
    };

    // This might fail on some filesystems, which is acceptable
    if write
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok()
    {
        assert!(std::path::Path::new(&long_filename).exists());
        fs::remove_file(&long_filename).ok();
    }

    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_path_with_spaces() {
    let test_dir = "tmp_rovodev_with spaces";
    fs::create_dir_all(test_dir).ok();

    let file_path = format!("{}/file with spaces.txt", test_dir);

    let write = Tool::WriteFile {
        path: file_path.clone(),
        content: "Content".to_string(),
    };
    assert!(write
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    let read = Tool::ReadFile {
        path: file_path.clone(),
    };
    assert!(read
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_search_replace_with_special_chars() {
    let test_file = "tmp_rovodev_special_replace.txt";

    let content = "Price: $100.00\nDiscount: 20%\nTotal: $80.00";

    let write = Tool::WriteFile {
        path: test_file.to_string(),
        content: content.to_string(),
    };
    assert!(write
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    let replace = Tool::SearchReplace {
        path: test_file.to_string(),
        old_string: "$100.00".to_string(),
        new_string: "$150.00".to_string(),
    };
    assert!(replace
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    let read = Tool::ReadFile {
        path: test_file.to_string(),
    };
    let result = read.execute(&rust_tui_coder::config::WebConfig::default());
    assert!(result.is_ok());
    assert!(result.unwrap().contains("$150.00"));

    fs::remove_file(test_file).ok();
}

#[test]
fn test_search_replace_not_found() {
    let test_file = "tmp_rovodev_not_found.txt";

    let write = Tool::WriteFile {
        path: test_file.to_string(),
        content: "Hello World".to_string(),
    };
    assert!(write
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    let replace = Tool::SearchReplace {
        path: test_file.to_string(),
        old_string: "NonExistent".to_string(),
        new_string: "Replacement".to_string(),
    };

    // Should fail because old string doesn't exist
    assert!(replace
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_err());

    fs::remove_file(test_file).ok();
}

#[test]
fn test_multiple_streaming_sessions() {
    let mut app = App::new();

    // First streaming session
    app.start_streaming();
    app.update_streaming_message("First message");
    app.finish_streaming("First message".to_string());

    // Second streaming session
    app.start_streaming();
    app.update_streaming_message("Second message");
    app.finish_streaming("Second message".to_string());

    // Third streaming session
    app.start_streaming();
    app.update_streaming_message("Third message");
    app.finish_streaming("Third message".to_string());

    assert_eq!(app.conversation.len(), 3);
    assert!(app.conversation[0].contains("First message"));
    assert!(app.conversation[1].contains("Second message"));
    assert!(app.conversation[2].contains("Third message"));
}

#[test]
fn test_token_estimation_edge_cases() {
    // Empty string
    assert_eq!(estimate_token_count(""), 0);

    // Single character
    assert_eq!(estimate_token_count("a"), 1);

    // Very long text
    let long_text = "word ".repeat(1000);
    let count = estimate_token_count(&long_text);
    assert!(count > 1000, "Should estimate tokens for long text");

    // Unicode
    let unicode = "🦀".repeat(100);
    let unicode_count = estimate_token_count(&unicode);
    assert!(unicode_count > 0, "Should handle unicode");
}

#[test]
fn test_app_state_after_error() {
    let mut app = App::new();

    // Simulate an error condition
    app.status_message = "Error occurred".to_string();
    app.is_executing_tool = true;

    // App should still function
    app.scroll_conversation_down();
    app.scroll_conversation_up();
    app.add_tool_log("Recovery attempt".to_string());

    // Reset state
    app.is_executing_tool = false;
    app.status_message = "Recovered".to_string();

    assert!(!app.is_executing_tool);
    assert_eq!(app.status_message, "Recovered");
}

#[test]
fn test_concurrent_scroll_and_update() {
    let mut app = App::new();

    // Add messages while scrolling
    for i in 0..50 {
        app.conversation.push(format!("Message {}", i));

        if i % 5 == 0 {
            app.scroll_conversation_down();
        }
    }

    assert_eq!(app.conversation.len(), 50);
    assert!(app.conversation_scroll_position > 0);
}

#[test]
fn test_delete_directory_with_content() {
    let test_dir = "tmp_rovodev_delete_dir";

    // Create directory with files
    fs::create_dir_all(format!("{}/subdir", test_dir)).ok();
    fs::write(format!("{}/file1.txt", test_dir), "content").ok();
    fs::write(format!("{}/subdir/file2.txt", test_dir), "content").ok();

    // Delete entire directory
    let delete = Tool::DeleteFile {
        path: test_dir.to_string(),
    };
    assert!(delete
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    assert!(!std::path::Path::new(test_dir).exists());
}

#[test]
fn test_command_with_error() {
    let cmd = Tool::RunCommand {
        command: "exit 1".to_string(),
    };

    let result = cmd.execute(&rust_tui_coder::config::WebConfig::default());
    // Command executes but returns failure status
    assert!(result.is_ok());
    assert!(result.unwrap().contains("failed"));
}

#[test]
fn test_plan_with_empty_steps() {
    let create = Tool::CreatePlan {
        task: "Empty task".to_string(),
        steps: vec![],
    };

    let result = create.execute(&rust_tui_coder::config::WebConfig::default());
    assert!(result.is_ok());

    let content = fs::read_to_string("plan.md").unwrap();
    assert!(content.contains("Empty task"));

    fs::remove_file("plan.md").ok();
}

#[test]
fn test_update_nonexistent_plan_step() {
    // Create plan with 2 steps
    let create = Tool::CreatePlan {
        task: "Test".to_string(),
        steps: vec!["Step 1".to_string(), "Step 2".to_string()],
    };
    create
        .execute(&rust_tui_coder::config::WebConfig::default())
        .ok();

    // Try to update step 5 (doesn't exist)
    let update = Tool::UpdatePlan { completed_step: 5 };
    let result = update.execute(&rust_tui_coder::config::WebConfig::default());

    // Should still succeed but won't find the step
    assert!(result.is_ok());

    fs::remove_file("plan.md").ok();
}

#[test]
fn test_usage_summary_with_zero_requests() {
    let app = App::new();

    let summary = app.get_usage_summary();

    // Should not divide by zero
    assert!(summary.contains("Average Tokens/Request: 0"));
}

#[test]
fn test_scroll_position_overflow_protection() {
    let mut app = App::new();

    // Set to maximum
    app.conversation_scroll_position = usize::MAX;

    // Scrolling down should not overflow
    app.scroll_conversation_down();
    assert_eq!(app.conversation_scroll_position, usize::MAX);

    // Scrolling up should work
    app.scroll_conversation_up();
    assert!(app.conversation_scroll_position < usize::MAX);
}

#[test]
fn test_nested_path_creation() {
    let nested_path = "tmp_rovodev_a/b/c/d/e/f/file.txt";

    let write = Tool::WriteFile {
        path: nested_path.to_string(),
        content: "Nested content".to_string(),
    };

    assert!(write
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());
    assert!(std::path::Path::new(nested_path).exists());

    fs::remove_dir_all("tmp_rovodev_a").ok();
}

#[test]
fn test_list_empty_directory() {
    let test_dir = "tmp_rovodev_empty_dir";
    fs::create_dir_all(test_dir).ok();

    let list = Tool::ListFiles {
        path: test_dir.to_string(),
    };

    let result = list.execute(&rust_tui_coder::config::WebConfig::default());
    assert!(result.is_ok());

    fs::remove_dir_all(test_dir).ok();
}
