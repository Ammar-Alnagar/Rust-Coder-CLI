// Separate test file for plan-related tests to avoid race conditions
// All tests in this file will be run sequentially
use rust_tui_coder::agent::Tool;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

// Global mutex to ensure plan tests run one at a time
static PLAN_TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_tool_create_plan() {
    let _lock = PLAN_TEST_MUTEX.lock().unwrap();

    // Ensure plan.md doesn't exist before test
    let _ = fs::remove_file("plan.md");

    let tool = Tool::CreatePlan {
        task: "Test Task".to_string(),
        steps: vec![
            "Step 1".to_string(),
            "Step 2".to_string(),
            "Step 3".to_string(),
        ],
    };

    let result = tool.execute();
    assert!(result.is_ok(), "CreatePlan should succeed: {:?}", result);

    // Add a small delay to ensure file system operations complete
    std::thread::sleep(std::time::Duration::from_millis(50));

    assert!(
        Path::new("plan.md").exists(),
        "plan.md should exist after CreatePlan"
    );

    let content = fs::read_to_string("plan.md").unwrap();
    assert!(
        content.contains("Test Task"),
        "Plan should contain task name"
    );
    assert!(content.contains("Step 1"), "Plan should contain Step 1");
    assert!(content.contains("Step 2"), "Plan should contain Step 2");
    assert!(content.contains("Step 3"), "Plan should contain Step 3");

    // Cleanup
    fs::remove_file("plan.md").ok();
}

#[test]
fn test_tool_update_plan() {
    let _lock = PLAN_TEST_MUTEX.lock().unwrap();

    // Ensure clean state
    let _ = fs::remove_file("plan.md");

    // First create a plan
    let create_tool = Tool::CreatePlan {
        task: "Test Task".to_string(),
        steps: vec!["Step 1".to_string(), "Step 2".to_string()],
    };
    let create_result = create_tool.execute();
    assert!(
        create_result.is_ok(),
        "CreatePlan should succeed: {:?}",
        create_result
    );

    // Add a small delay to ensure file system operations complete
    std::thread::sleep(std::time::Duration::from_millis(50));

    assert!(
        Path::new("plan.md").exists(),
        "plan.md should exist after CreatePlan"
    );

    // Then update it
    let update_tool = Tool::UpdatePlan { completed_step: 1 };
    let result = update_tool.execute();
    assert!(result.is_ok(), "UpdatePlan should succeed: {:?}", result);

    let content = fs::read_to_string("plan.md").unwrap();
    assert!(
        content.contains("[x]"),
        "Plan should contain completed checkbox"
    );

    // Cleanup
    fs::remove_file("plan.md").ok();
}

#[test]
fn test_tool_clear_plan() {
    let _lock = PLAN_TEST_MUTEX.lock().unwrap();

    // Ensure clean state
    let _ = fs::remove_file("plan.md");

    // First create a plan
    let create_tool = Tool::CreatePlan {
        task: "Test Task".to_string(),
        steps: vec!["Step 1".to_string()],
    };
    let create_result = create_tool.execute();
    assert!(
        create_result.is_ok(),
        "CreatePlan should succeed: {:?}",
        create_result
    );

    // Add a small delay to ensure file system operations complete
    std::thread::sleep(std::time::Duration::from_millis(50));

    assert!(
        Path::new("plan.md").exists(),
        "plan.md should exist after CreatePlan"
    );

    // Then clear it
    let clear_tool = Tool::ClearPlan;
    let result = clear_tool.execute();
    assert!(result.is_ok(), "ClearPlan should succeed: {:?}", result);

    // Add a small delay to ensure file system operations complete
    std::thread::sleep(std::time::Duration::from_millis(50));

    assert!(
        !Path::new("plan.md").exists(),
        "plan.md should not exist after ClearPlan"
    );
}

#[test]
fn test_plan_lifecycle() {
    let _lock = PLAN_TEST_MUTEX.lock().unwrap();

    // Ensure clean state
    let _ = fs::remove_file("plan.md");

    // Create plan
    let create = Tool::CreatePlan {
        task: "Complete project".to_string(),
        steps: vec![
            "Setup environment".to_string(),
            "Write code".to_string(),
            "Test code".to_string(),
            "Deploy".to_string(),
        ],
    };
    assert!(create.execute().is_ok(), "CreatePlan should succeed");

    std::thread::sleep(std::time::Duration::from_millis(50));

    // Update each step
    for i in 1..=4 {
        let update = Tool::UpdatePlan { completed_step: i };
        assert!(
            update.execute().is_ok(),
            "UpdatePlan step {} should succeed",
            i
        );
    }

    // Verify completion
    let content = fs::read_to_string("plan.md").unwrap();
    assert!(
        content.contains("Completed: 4"),
        "Plan should show 4 completed steps"
    );

    // Clear plan
    let clear = Tool::ClearPlan;
    assert!(clear.execute().is_ok(), "ClearPlan should succeed");

    std::thread::sleep(std::time::Duration::from_millis(50));

    assert!(
        !Path::new("plan.md").exists(),
        "plan.md should be deleted after ClearPlan"
    );
}

#[test]
fn test_plan_with_empty_steps() {
    let _lock = PLAN_TEST_MUTEX.lock().unwrap();

    // Ensure clean state
    let _ = fs::remove_file("plan.md");

    // Create plan with no steps
    let create = Tool::CreatePlan {
        task: "Task with no steps".to_string(),
        steps: vec![],
    };
    let result = create.execute();
    assert!(result.is_ok(), "CreatePlan with empty steps should succeed");

    std::thread::sleep(std::time::Duration::from_millis(50));

    assert!(Path::new("plan.md").exists(), "plan.md should exist");

    let content = fs::read_to_string("plan.md").unwrap();
    assert!(
        content.contains("Task with no steps"),
        "Plan should contain task name"
    );

    // Cleanup
    fs::remove_file("plan.md").ok();
}

#[test]
fn test_plan_update_nonexistent_step() {
    let _lock = PLAN_TEST_MUTEX.lock().unwrap();

    // Ensure clean state
    let _ = fs::remove_file("plan.md");

    // Create plan with 2 steps
    let create = Tool::CreatePlan {
        task: "Test Task".to_string(),
        steps: vec!["Step 1".to_string(), "Step 2".to_string()],
    };
    assert!(create.execute().is_ok(), "CreatePlan should succeed");

    std::thread::sleep(std::time::Duration::from_millis(50));

    // Try to update step 5 (doesn't exist)
    let update = Tool::UpdatePlan { completed_step: 5 };
    let result = update.execute();
    // Should still succeed but not mark anything
    assert!(
        result.is_ok(),
        "UpdatePlan should succeed even for nonexistent step"
    );

    // Cleanup
    fs::remove_file("plan.md").ok();
}

#[test]
fn test_clear_nonexistent_plan() {
    let _lock = PLAN_TEST_MUTEX.lock().unwrap();

    // Ensure no plan exists
    let _ = fs::remove_file("plan.md");

    // Try to clear non-existent plan
    let clear = Tool::ClearPlan;
    let result = clear.execute();
    assert!(
        result.is_ok(),
        "ClearPlan should succeed even if plan doesn't exist"
    );
    assert!(
        result.unwrap().contains("No plan.md file found"),
        "Should indicate no file found"
    );
}

#[test]
fn test_update_plan_before_create() {
    let _lock = PLAN_TEST_MUTEX.lock().unwrap();

    // Ensure no plan exists
    let _ = fs::remove_file("plan.md");

    // Try to update without creating first
    let update = Tool::UpdatePlan { completed_step: 1 };
    let result = update.execute();
    assert!(
        result.is_err(),
        "UpdatePlan should fail if plan doesn't exist"
    );
}

#[test]
fn test_plan_with_special_characters() {
    let _lock = PLAN_TEST_MUTEX.lock().unwrap();

    // Ensure clean state
    let _ = fs::remove_file("plan.md");

    // Create plan with special characters
    let create = Tool::CreatePlan {
        task: "Task with émojis 🚀 and spëcial çharacters!".to_string(),
        steps: vec![
            "Step with <brackets>".to_string(),
            "Step with [square] brackets".to_string(),
            "Step with 'quotes'".to_string(),
        ],
    };
    let result = create.execute();
    assert!(
        result.is_ok(),
        "CreatePlan with special characters should succeed"
    );

    std::thread::sleep(std::time::Duration::from_millis(50));

    let content = fs::read_to_string("plan.md").unwrap();
    assert!(content.contains("🚀"), "Plan should preserve emojis");
    assert!(
        content.contains("<brackets>"),
        "Plan should preserve angle brackets"
    );

    // Cleanup
    fs::remove_file("plan.md").ok();
}
