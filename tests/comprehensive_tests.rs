use rust_tui_coder::agent::Tool;
use rust_tui_coder::app::App;
use std::fs;

#[test]
fn test_end_to_end_file_operations() {
    let test_dir = "tmp_rovodev_e2e_test";

    // Create directory
    let create_dir = Tool::CreateDirectory {
        path: test_dir.to_string(),
    };
    assert!(create_dir
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    // Write multiple files
    for i in 1..=3 {
        let write = Tool::WriteFile {
            path: format!("{}/file{}.txt", test_dir, i),
            content: format!("Content for file {}", i),
        };
        assert!(write
            .execute(&rust_tui_coder::config::WebConfig::default())
            .is_ok());
    }

    // List files
    let list = Tool::ListFiles {
        path: test_dir.to_string(),
    };
    let result = list.execute(&rust_tui_coder::config::WebConfig::default());
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("file1.txt"));
    assert!(output.contains("file2.txt"));
    assert!(output.contains("file3.txt"));

    // Read and verify
    let read = Tool::ReadFile {
        path: format!("{}/file1.txt", test_dir),
    };
    let result = read.execute(&rust_tui_coder::config::WebConfig::default());
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Content for file 1"));

    // Modify file
    let replace = Tool::SearchReplace {
        path: format!("{}/file1.txt", test_dir),
        old_string: "Content".to_string(),
        new_string: "Modified".to_string(),
    };
    assert!(replace
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    // Verify modification
    let read2 = Tool::ReadFile {
        path: format!("{}/file1.txt", test_dir),
    };
    let result2 = read2.execute(&rust_tui_coder::config::WebConfig::default());
    assert!(result2.is_ok());
    assert!(result2.unwrap().contains("Modified for file 1"));

    // Clean up
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_app_state_transitions() {
    let mut app = App::new();

    // Initial state
    assert_eq!(app.conversation.len(), 0);
    assert_eq!(app.tokens_used, 0);
    assert!(!app.is_streaming);

    // Add conversation
    app.conversation.push("User: Start task".to_string());

    // Start streaming
    app.start_streaming();
    assert!(app.is_streaming);

    // Update streaming content
    app.update_streaming_message("Processing");
    app.update_streaming_message("...");
    assert_eq!(app.current_streaming_message, "Processing...");

    // Finish streaming
    app.finish_streaming("Processing...".to_string());
    assert!(!app.is_streaming);
    assert_eq!(app.conversation.len(), 2);

    // Update stats
    app.increment_tokens(500);
    app.increment_requests();
    app.increment_tools_executed();

    assert_eq!(app.tokens_used, 500);
    assert_eq!(app.total_requests, 1);
    assert_eq!(app.total_tools_executed, 1);

    // Get summary
    let summary = app.get_usage_summary();
    assert!(summary.contains("500"));
    assert!(summary.contains("1"));
}

#[test]
fn test_nested_directory_operations() {
    let base_dir = "tmp_rovodev_nested";
    let nested_path = format!("{}/level1/level2/level3", base_dir);

    // Create deeply nested directory
    let create = Tool::CreateDirectory {
        path: nested_path.clone(),
    };
    assert!(create
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    // Write file in nested location
    let write = Tool::WriteFile {
        path: format!("{}/deep_file.txt", nested_path),
        content: "Deep content".to_string(),
    };
    assert!(write
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    // List recursively
    let list_recursive = Tool::ListFilesRecursive {
        path: base_dir.to_string(),
    };
    let result = list_recursive.execute(&rust_tui_coder::config::WebConfig::default());
    assert!(result.is_ok());
    assert!(result.unwrap().contains("deep_file.txt"));

    // Clean up
    fs::remove_dir_all(base_dir).ok();
}

#[test]
fn test_command_execution() {
    // Test simple command
    let cmd = Tool::RunCommand {
        command: "echo 'test output'".to_string(),
    };
    let result = cmd.execute(&rust_tui_coder::config::WebConfig::default());
    assert!(result.is_ok());
    assert!(result.unwrap().contains("test output"));

    // Test command with pipes
    let cmd2 = Tool::RunCommand {
        command: "echo 'hello' | tr 'h' 'H'".to_string(),
    };
    let result2 = cmd2.execute(&rust_tui_coder::config::WebConfig::default());
    assert!(result2.is_ok());
    assert!(result2.unwrap().contains("Hello"));
}

#[test]
fn test_file_append_multiple_times() {
    let test_file = "tmp_rovodev_append_multi.txt";

    // Create initial file
    let write = Tool::WriteFile {
        path: test_file.to_string(),
        content: "Line 1\n".to_string(),
    };
    assert!(write
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    // Append multiple times
    for i in 2..=5 {
        let append = Tool::AppendFile {
            path: test_file.to_string(),
            content: format!("Line {}\n", i),
        };
        assert!(append
            .execute(&rust_tui_coder::config::WebConfig::default())
            .is_ok());
    }

    // Verify all lines present
    let read = Tool::ReadFile {
        path: test_file.to_string(),
    };
    let result = read.execute(&rust_tui_coder::config::WebConfig::default());
    assert!(result.is_ok());
    let content = result.unwrap();

    for i in 1..=5 {
        assert!(content.contains(&format!("Line {}", i)));
    }

    fs::remove_file(test_file).ok();
}

#[test]
fn test_search_replace_multiple_occurrences() {
    let test_file = "tmp_rovodev_replace_multi.txt";

    let content = "foo bar foo baz foo";
    let write = Tool::WriteFile {
        path: test_file.to_string(),
        content: content.to_string(),
    };
    assert!(write
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    // Replace all occurrences
    let replace = Tool::SearchReplace {
        path: test_file.to_string(),
        old_string: "foo".to_string(),
        new_string: "FOO".to_string(),
    };
    assert!(replace
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    // Verify all replaced
    let read = Tool::ReadFile {
        path: test_file.to_string(),
    };
    let result = read.execute(&rust_tui_coder::config::WebConfig::default());
    assert!(result.is_ok());
    let new_content = result.unwrap();
    assert!(new_content.contains("FOO bar FOO baz FOO"));
    assert!(!new_content.contains("foo"));

    fs::remove_file(test_file).ok();
}

#[test]
fn test_tool_error_handling() {
    // Try to read non-existent file
    let read = Tool::ReadFile {
        path: "tmp_rovodev_nonexistent.txt".to_string(),
    };
    assert!(read
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_err());

    // Try to append to non-existent file
    let append = Tool::AppendFile {
        path: "tmp_rovodev_nonexistent.txt".to_string(),
        content: "content".to_string(),
    };
    assert!(append
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_err());

    // Try to replace in non-existent file
    let replace = Tool::SearchReplace {
        path: "tmp_rovodev_nonexistent.txt".to_string(),
        old_string: "old".to_string(),
        new_string: "new".to_string(),
    };
    assert!(replace
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_err());
}

// Plan lifecycle test moved to tests/plan_tests.rs to avoid race conditions

#[test]
fn test_usage_tracking_accuracy() {
    let mut app = App::new();

    // Track various operations
    for _ in 0..10 {
        app.increment_requests();
    }

    for _ in 0..25 {
        app.increment_tools_executed();
    }

    app.increment_tokens(1000);
    app.increment_tokens(500);
    app.increment_tokens(1500);

    assert_eq!(app.total_requests, 10);
    assert_eq!(app.total_tools_executed, 25);
    assert_eq!(app.tokens_used, 3000);

    // Check average
    let summary = app.get_usage_summary();
    assert!(summary.contains("Average Tokens/Request: 300"));
}

#[test]
fn test_concurrent_file_operations() {
    let test_dir = "tmp_rovodev_concurrent";
    fs::create_dir_all(test_dir).ok();

    // Create multiple files quickly
    let files: Vec<String> = (1..=10)
        .map(|i| format!("{}/file{}.txt", test_dir, i))
        .collect();

    for (i, file) in files.iter().enumerate() {
        let write = Tool::WriteFile {
            path: file.clone(),
            content: format!("Content {}", i + 1),
        };
        assert!(write
            .execute(&rust_tui_coder::config::WebConfig::default())
            .is_ok());
    }

    // Verify all exist
    for file in &files {
        assert!(std::path::Path::new(file).exists());
    }

    // Clean up
    fs::remove_dir_all(test_dir).ok();
}
