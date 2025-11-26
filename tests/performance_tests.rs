use rust_tui_coder::agent::Tool;
use rust_tui_coder::app::App;
use std::fs;
use std::time::Instant;

#[test]
fn test_large_conversation_handling() {
    let mut app = App::new();

    let start = Instant::now();

    // Add 1000 messages
    for i in 0..1000 {
        app.conversation.push(format!("User: Message {}", i));
        app.conversation
            .push(format!("Agent: Response to message {}", i));
    }

    let elapsed = start.elapsed();

    assert_eq!(app.conversation.len(), 2000);
    assert!(elapsed.as_secs() < 1, "Should handle 2000 messages quickly");
}

#[test]
fn test_rapid_scrolling() {
    let mut app = App::new();

    // Add messages
    for i in 0..100 {
        app.conversation.push(format!("Message {}", i));
    }

    let start = Instant::now();

    // Rapid scrolling operations
    for _ in 0..1000 {
        app.scroll_conversation_down();
        app.scroll_conversation_up();
    }

    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() < 100, "Scrolling should be fast");
}

#[test]
fn test_large_file_operations() {
    let test_file = "tmp_rovodev_large_file.txt";

    // Create large content (1MB)
    let large_content: String = "x".repeat(1024 * 1024);

    let start = Instant::now();

    let write = Tool::WriteFile {
        path: test_file.to_string(),
        content: large_content.clone(),
    };
    assert!(write
        .execute(&rust_tui_coder::config::WebConfig::default())
        .is_ok());

    let write_elapsed = start.elapsed();

    let read = Tool::ReadFile {
        path: test_file.to_string(),
    };

    let read_start = Instant::now();
    let result = read.execute(&rust_tui_coder::config::WebConfig::default());
    let read_elapsed = read_start.elapsed();

    assert!(result.is_ok());
    assert!(write_elapsed.as_secs() < 2, "Writing 1MB should be fast");
    assert!(read_elapsed.as_secs() < 2, "Reading 1MB should be fast");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_many_tool_logs() {
    let mut app = App::new();

    let start = Instant::now();

    // Add 10000 tool logs
    for i in 0..10000 {
        app.add_tool_log(format!("Tool execution {}: completed", i));
    }

    let elapsed = start.elapsed();

    assert_eq!(app.tool_logs.len(), 10000);
    assert!(elapsed.as_millis() < 500, "Adding tool logs should be fast");
}

#[test]
fn test_streaming_performance() {
    let mut app = App::new();

    let start = Instant::now();

    app.start_streaming();

    // Simulate streaming 10000 small chunks
    for i in 0..10000 {
        app.update_streaming_message(&format!("{} ", i));
    }

    app.finish_streaming(app.current_streaming_message.clone());

    let elapsed = start.elapsed();

    assert!(
        elapsed.as_secs() < 1,
        "Streaming should handle many chunks efficiently"
    );
}

#[test]
fn test_usage_tracking_performance() {
    let mut app = App::new();

    let start = Instant::now();

    // Perform many tracking operations
    for _ in 0..100000 {
        app.increment_tokens(10);
        app.increment_requests();
        app.increment_tools_executed();
    }

    let elapsed = start.elapsed();

    assert_eq!(app.tokens_used, 1000000);
    assert_eq!(app.total_requests, 100000);
    assert_eq!(app.total_tools_executed, 100000);
    assert!(elapsed.as_millis() < 100, "Tracking should be very fast");
}

#[test]
fn test_directory_with_many_files() {
    let test_dir = "tmp_rovodev_many_files";
    fs::create_dir_all(test_dir).ok();

    // Create 100 files
    for i in 0..100 {
        let write = Tool::WriteFile {
            path: format!("{}/file{}.txt", test_dir, i),
            content: format!("Content {}", i),
        };
        write
            .execute(&rust_tui_coder::config::WebConfig::default())
            .ok();
    }

    let start = Instant::now();

    let list = Tool::ListFiles {
        path: test_dir.to_string(),
    };
    let result = list.execute(&rust_tui_coder::config::WebConfig::default());

    let elapsed = start.elapsed();

    assert!(result.is_ok());
    assert!(
        elapsed.as_millis() < 500,
        "Listing 100 files should be fast"
    );

    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_recursive_directory_listing_performance() {
    let test_dir = "tmp_rovodev_recursive_perf";

    // Create nested structure
    for level1 in 0..5 {
        for level2 in 0..5 {
            let dir_path = format!("{}/dir{}/subdir{}", test_dir, level1, level2);
            fs::create_dir_all(&dir_path).ok();

            for file in 0..5 {
                let write = Tool::WriteFile {
                    path: format!("{}/file{}.txt", dir_path, file),
                    content: "content".to_string(),
                };
                write
                    .execute(&rust_tui_coder::config::WebConfig::default())
                    .ok();
            }
        }
    }

    let start = Instant::now();

    let list = Tool::ListFilesRecursive {
        path: test_dir.to_string(),
    };
    let result = list.execute(&rust_tui_coder::config::WebConfig::default());

    let elapsed = start.elapsed();

    assert!(result.is_ok());
    assert!(
        elapsed.as_secs() < 2,
        "Recursive listing should complete in reasonable time"
    );

    fs::remove_dir_all(test_dir).ok();
}
