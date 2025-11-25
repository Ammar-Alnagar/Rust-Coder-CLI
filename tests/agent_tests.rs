use rust_tui_coder::agent::{Agent, Tool};
use std::fs;
use std::path::Path;

#[test]
fn test_agent_new() {
    let agent = Agent::new();
    // Agent is created successfully
    assert!(true); // Agent has private fields, so we can only test creation
}

#[test]
fn test_agent_default() {
    let _agent = Agent::default();
    assert!(true); // Agent is created successfully
}

#[test]
fn test_tool_read_file() {
    let test_file = "tmp_rovodev_test_read.txt";
    fs::write(test_file, "Test content").unwrap();
    
    let tool = Tool::ReadFile {
        path: test_file.to_string(),
    };
    
    let result = tool.execute();
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Test content"));
    
    fs::remove_file(test_file).ok();
}

#[test]
fn test_tool_write_file() {
    let test_file = "tmp_rovodev_test_write.txt";
    
    let tool = Tool::WriteFile {
        path: test_file.to_string(),
        content: "Hello World!".to_string(),
    };
    
    let result = tool.execute();
    assert!(result.is_ok());
    assert!(Path::new(test_file).exists());
    
    let content = fs::read_to_string(test_file).unwrap();
    assert_eq!(content, "Hello World!");
    
    fs::remove_file(test_file).ok();
}

#[test]
fn test_tool_append_file() {
    let test_file = "tmp_rovodev_test_append.txt";
    fs::write(test_file, "Initial content\n").unwrap();
    
    let tool = Tool::AppendFile {
        path: test_file.to_string(),
        content: "Appended content".to_string(),
    };
    
    let result = tool.execute();
    assert!(result.is_ok());
    
    let content = fs::read_to_string(test_file).unwrap();
    assert!(content.contains("Initial content"));
    assert!(content.contains("Appended content"));
    
    fs::remove_file(test_file).ok();
}

#[test]
fn test_tool_search_replace() {
    let test_file = "tmp_rovodev_test_search_replace.txt";
    fs::write(test_file, "Hello World! World is great.").unwrap();
    
    let tool = Tool::SearchReplace {
        path: test_file.to_string(),
        old_string: "World".to_string(),
        new_string: "Rust".to_string(),
    };
    
    let result = tool.execute();
    assert!(result.is_ok());
    
    let content = fs::read_to_string(test_file).unwrap();
    assert_eq!(content, "Hello Rust! Rust is great.");
    
    fs::remove_file(test_file).ok();
}

#[test]
fn test_tool_delete_file() {
    let test_file = "tmp_rovodev_test_delete.txt";
    fs::write(test_file, "To be deleted").unwrap();
    
    let tool = Tool::DeleteFile {
        path: test_file.to_string(),
    };
    
    let result = tool.execute();
    assert!(result.is_ok());
    assert!(!Path::new(test_file).exists());
}

#[test]
fn test_tool_create_directory() {
    let test_dir = "tmp_rovodev_test_dir";
    
    let tool = Tool::CreateDirectory {
        path: test_dir.to_string(),
    };
    
    let result = tool.execute();
    assert!(result.is_ok());
    assert!(Path::new(test_dir).is_dir());
    
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_tool_list_files() {
    let test_dir = "tmp_rovodev_test_list";
    fs::create_dir_all(test_dir).unwrap();
    fs::write(format!("{}/file1.txt", test_dir), "content1").unwrap();
    fs::write(format!("{}/file2.txt", test_dir), "content2").unwrap();
    fs::create_dir(format!("{}/subdir", test_dir)).unwrap();
    
    let tool = Tool::ListFiles {
        path: test_dir.to_string(),
    };
    
    let result = tool.execute();
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("file1.txt"));
    assert!(output.contains("file2.txt"));
    assert!(output.contains("subdir"));
    
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_tool_list_files_recursive() {
    let test_dir = "tmp_rovodev_test_recursive";
    fs::create_dir_all(format!("{}/subdir", test_dir)).unwrap();
    fs::write(format!("{}/file1.txt", test_dir), "content1").unwrap();
    fs::write(format!("{}/subdir/file2.txt", test_dir), "content2").unwrap();
    
    let tool = Tool::ListFilesRecursive {
        path: test_dir.to_string(),
    };
    
    let result = tool.execute();
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("file1.txt"));
    assert!(output.contains("file2.txt"));
    
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_tool_run_command() {
    let tool = Tool::RunCommand {
        command: "echo 'Hello from test'".to_string(),
    };
    
    let result = tool.execute();
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Hello from test"));
}

#[test]
fn test_tool_execute_code_python() {
    let tool = Tool::ExecuteCode {
        language: "python".to_string(),
        code: "print('Python test')".to_string(),
    };
    
    let result = tool.execute();
    // Python might not be available in all test environments
    if result.is_ok() {
        assert!(result.unwrap().contains("Python test"));
    }
}

#[test]
fn test_tool_execute_code_bash() {
    let tool = Tool::ExecuteCode {
        language: "bash".to_string(),
        code: "echo 'Bash test'".to_string(),
    };
    
    let result = tool.execute();
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Bash test"));
}

#[test]
fn test_tool_create_plan() {
    let tool = Tool::CreatePlan {
        task: "Test Task".to_string(),
        steps: vec![
            "Step 1".to_string(),
            "Step 2".to_string(),
            "Step 3".to_string(),
        ],
    };
    
    let result = tool.execute();
    assert!(result.is_ok());
    assert!(Path::new("plan.md").exists());
    
    let content = fs::read_to_string("plan.md").unwrap();
    assert!(content.contains("Test Task"));
    assert!(content.contains("Step 1"));
    assert!(content.contains("Step 2"));
    assert!(content.contains("Step 3"));
    
    fs::remove_file("plan.md").ok();
}

#[test]
fn test_tool_update_plan() {
    // First create a plan
    let create_tool = Tool::CreatePlan {
        task: "Test Task".to_string(),
        steps: vec!["Step 1".to_string(), "Step 2".to_string()],
    };
    create_tool.execute().unwrap();
    
    // Then update it
    let update_tool = Tool::UpdatePlan { completed_step: 1 };
    let result = update_tool.execute();
    assert!(result.is_ok());
    
    let content = fs::read_to_string("plan.md").unwrap();
    assert!(content.contains("[x]"));
    
    fs::remove_file("plan.md").ok();
}

#[test]
fn test_tool_clear_plan() {
    // First create a plan
    let create_tool = Tool::CreatePlan {
        task: "Test Task".to_string(),
        steps: vec!["Step 1".to_string()],
    };
    create_tool.execute().unwrap();
    
    // Then clear it
    let clear_tool = Tool::ClearPlan;
    let result = clear_tool.execute();
    assert!(result.is_ok());
    assert!(!Path::new("plan.md").exists());
}

#[test]
fn test_tool_git_status() {
    let tool = Tool::GitStatus;
    let result = tool.execute();
    // Git might not be available or this might not be a git repo
    assert!(result.is_ok());
}
