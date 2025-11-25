# Testing Documentation

This document describes the comprehensive test suite for the Rust TUI Coder project.

## Running Tests

Run all tests:
```bash
cargo test
```

Run specific test suites:
```bash
cargo test --test agent_tests      # Agent and tool tests
cargo test --test app_tests         # Application state tests
cargo test --test config_tests      # Configuration tests
cargo test --test integration_tests # Integration tests
cargo test --test llm_tests         # LLM module tests
cargo test --test ui_tests          # UI and scrolling tests
cargo test --test plan_tests        # Plan management tests (synchronized)
cargo test --test comprehensive_tests # End-to-end tests
cargo test --test performance_tests # Performance benchmarks
cargo test --test edge_case_tests   # Edge case handling
```

## Test Coverage

### 1. Agent Tests (`tests/agent_tests.rs`)
- **14 tests** covering basic tool operations:
  - File operations (read, write, append, search/replace, delete)
  - Directory operations (create, list, recursive list)
  - Code execution (Python, Bash)
  - Git operations (status)
  - Command execution

### 2. App Tests (`tests/app_tests.rs`)
- **13 tests** for application state:
  - App initialization and defaults
  - Tool log management
  - Token/request/tool tracking
  - Scrolling behavior (up, down, to top, to bottom)
  - Streaming state management
  - Usage summary generation
  - Session duration tracking

### 3. Config Tests (`tests/config_tests.rs`)
- **4 tests** for configuration:
  - Loading from TOML file
  - Missing file handling
  - Invalid TOML handling
  - Config cloning

### 4. Integration Tests (`tests/integration_tests.rs`)
- **5 tests** for complete workflows:
  - Full file operation workflow
  - App state workflow
  - Plan management workflow
  - Streaming workflow
  - Multiple file operations

### 5. LLM Tests (`tests/llm_tests.rs`)
- **9 tests** for LLM functionality:
  - Token count estimation (empty, short, long, whitespace, unicode)
  - Message creation and cloning
  - Message serialization/deserialization

### 6. UI Tests (`tests/ui_tests.rs`)
- **9 tests** for UI behavior:
  - Scrolling behavior and boundaries
  - Scroll position clamping
  - Conversation display
  - Tool logs display
  - Streaming state management
  - Page scrolling
  - Status message updates
  - Empty conversation handling
  - Tool execution state

### 7. Comprehensive Tests (`tests/comprehensive_tests.rs`)
- **10 tests** for complex scenarios:
  - End-to-end file operations
  - App state transitions
  - Nested directory operations
  - Command execution with pipes
  - Multiple file appends
  - Multiple search/replace operations
  - Error handling
  - Plan lifecycle
  - Usage tracking accuracy
  - Concurrent file operations

### 8. Performance Tests (`tests/performance_tests.rs`)
- **8 tests** for performance:
  - Large conversation handling (2000 messages)
  - Rapid scrolling (1000 operations)
  - Large file operations (1MB files)
  - Many tool logs (10,000 logs)
  - Streaming performance (10,000 chunks)
  - Usage tracking performance (100,000 operations)
  - Directory with many files (100 files)
  - Recursive directory listing

### 9. Plan Tests (`tests/plan_tests.rs`)
- **9 tests** for plan management with proper synchronization:
  - Create plan with steps
  - Update plan steps
  - Clear plan
  - Plan lifecycle (complete workflow)
  - Edge cases (empty steps, nonexistent steps, special characters)
  - Error handling (update before create, clear nonexistent)
  - **Note:** Uses mutex to prevent race conditions in parallel execution

### 10. Edge Case Tests (`tests/edge_case_tests.rs`)
- **19 tests** for edge cases:
  - Empty file operations
  - Special characters in content
  - Unicode content (emoji, non-Latin scripts)
  - Very long filenames
  - Paths with spaces
  - Search/replace with special characters
  - Search/replace not found errors
  - Multiple streaming sessions
  - Token estimation edge cases
  - App state after errors
  - Concurrent scroll and update
  - Delete directory with content
  - Commands with errors
  - Plans with empty steps
  - Update nonexistent plan steps
  - Zero division protection
  - Scroll position overflow protection
  - Nested path creation
  - Empty directory listing

## Total Test Count

**99 tests** across 10 test suites

## Test Naming Conventions

All temporary files created during tests use the prefix `tmp_rovodev_` to:
- Avoid conflicts with real project files
- Make cleanup easier
- Clearly identify test artifacts

## Continuous Integration

To ensure code quality:

1. **Run all tests:**
   ```bash
   cargo test
   ```

2. **Check for clippy warnings:**
   ```bash
   cargo clippy --all-targets --all-features
   ```

3. **Build in release mode:**
   ```bash
   cargo build --release
   ```

## Test Philosophy

- **Comprehensive Coverage:** Tests cover normal operations, edge cases, error conditions, and performance
- **Fast Execution:** Most tests complete in milliseconds
- **Isolated:** Tests clean up after themselves and use unique file names
- **Realistic:** Tests simulate real-world usage patterns
- **Maintainable:** Clear test names and documentation

## Known Test Limitations

1. Some tests (like git operations) may behave differently depending on the environment
2. Code execution tests require interpreters (Python, Node.js, etc.) to be installed
3. Performance tests have generous time limits to account for slower CI environments
4. Tests are designed to be run sequentially due to shared resources (like plan.md)

## Adding New Tests

When adding new tests:
1. Use the `tmp_rovodev_` prefix for any temporary files
2. Clean up resources in the test (use `.ok()` on cleanup to not fail on cleanup errors)
3. Add documentation to this file
4. Ensure tests are isolated and don't depend on execution order
5. Include both positive and negative test cases
