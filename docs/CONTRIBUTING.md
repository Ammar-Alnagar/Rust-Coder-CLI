# Contributing to Rust TUI Coder

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Setup](#development-setup)
4. [Project Structure](#project-structure)
5. [Making Changes](#making-changes)
6. [Testing](#testing)
7. [Code Style](#code-style)
8. [Submitting Changes](#submitting-changes)
9. [Adding Features](#adding-features)
10. [Reporting Bugs](#reporting-bugs)

---

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inspiring community for all. Please:

- Be respectful and inclusive
- Be patient with newcomers
- Focus on what is best for the community
- Show empathy towards others

### Unacceptable Behavior

- Harassment, discrimination, or offensive comments
- Trolling or insulting remarks
- Publishing others' private information
- Other conduct inappropriate for a professional setting

---

## Getting Started

### Prerequisites

- **Rust** 1.70 or higher
- **Git** for version control
- Basic understanding of:
  - Rust programming
  - Terminal/TUI applications
  - Async programming (Tokio)
  - REST APIs

### Find an Issue

1. Check the [Issues](../../issues) page
2. Look for issues labeled:
   - `good first issue` - Great for newcomers
   - `help wanted` - Community help needed
   - `bug` - Bug fixes
   - `enhancement` - New features

3. Comment on the issue to express interest
4. Wait for maintainer confirmation before starting work

---

## Development Setup

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/rust_tui_coder.git
cd rust_tui_coder
```

### 2. Build the Project

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test agent_tests

# Run with output
cargo test -- --nocapture
```

### 4. Run the Application

```bash
# Create a config file
cp config_example.toml config.toml
# Edit config.toml with your API key

# Run the application
cargo run
```

### 5. Check Code Quality

```bash
# Run clippy for lints
cargo clippy --all-targets --all-features

# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

---

## Project Structure

```
rust_tui_coder/
├── src/
│   ├── main.rs          # Entry point, event loop
│   ├── app.rs           # Application state
│   ├── ui.rs            # TUI rendering
│   ├── llm.rs           # LLM API interface
│   ├── agent.rs         # Tool execution
│   ├── config.rs        # Configuration loading
│   └── lib.rs           # Library exports
├── tests/
│   ├── agent_tests.rs   # Tool tests
│   ├── app_tests.rs     # App state tests
│   ├── config_tests.rs  # Config tests
│   ├── llm_tests.rs     # LLM tests
│   ├── ui_tests.rs      # UI tests
│   └── ...              # More test files
├── docs/
│   ├── README.md        # User documentation
│   ├── ARCHITECTURE.md  # System design
│   ├── API.md           # API reference
│   └── ...              # More documentation
├── Cargo.toml           # Project metadata
├── config_example.toml  # Example configuration
└── README.md            # Project overview (symlink to docs/README.md)
```

---

## Making Changes

### 1. Create a Branch

```bash
# Create a feature branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/issue-description
```

### 2. Make Your Changes

- Write clear, readable code
- Follow Rust conventions
- Add comments for complex logic
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run tests
cargo test

# Test your specific changes
cargo test test_name

# Run the app and test manually
cargo run
```

### 4. Commit Your Changes

```bash
# Stage changes
git add .

# Commit with descriptive message
git commit -m "feat: add new tool for database operations"
```

**Commit Message Format:**

```
<type>: <description>

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat: add execute_rust tool for running Rust code
fix: handle empty conversation scroll correctly
docs: update API documentation for new tools
test: add edge case tests for file operations
```

---

## Testing

### Writing Tests

All tests should follow these guidelines:

#### 1. Test File Naming

- Place tests in `tests/` directory
- Name files `{module}_tests.rs`
- Example: `agent_tests.rs`, `app_tests.rs`

#### 2. Test Naming Convention

```rust
#[tokio::test]
async fn test_tool_name_success_case() {
    // Test implementation
}

#[tokio::test]
async fn test_tool_name_error_case() {
    // Test implementation
}
```

#### 3. Use Temporary Files

Always use the `tmp_rovodev_` prefix for test files:

```rust
#[tokio::test]
async fn test_write_file() {
    let path = "tmp_rovodev_test.txt";
    
    // Test code here
    
    // Cleanup
    std::fs::remove_file(path).ok();
}
```

#### 4. Test Categories

Write tests for:
- **Happy path**: Normal, expected usage
- **Edge cases**: Boundary conditions, empty inputs
- **Error handling**: Invalid inputs, failures
- **Integration**: Component interactions

#### 5. Example Test

```rust
#[tokio::test]
async fn test_read_file_success() {
    // Setup
    let path = "tmp_rovodev_read_test.txt";
    let content = "test content";
    std::fs::write(path, content).unwrap();
    
    // Execute
    let args = json!({ "path": path });
    let result = agent::execute_tool("read_file", &args).await;
    
    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), content);
    
    // Cleanup
    std::fs::remove_file(path).ok();
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific suite
cargo test --test agent_tests

# Specific test
cargo test test_read_file_success

# With output
cargo test -- --nocapture

# With verbose output
cargo test -- --nocapture --test-threads=1
```

---

## Code Style

### Rust Style Guidelines

1. **Follow Rustfmt**
   ```bash
   cargo fmt
   ```

2. **Follow Clippy Suggestions**
   ```bash
   cargo clippy --all-targets --all-features
   ```

3. **Naming Conventions**
   - Functions: `snake_case`
   - Types: `PascalCase`
   - Constants: `SCREAMING_SNAKE_CASE`
   - Modules: `snake_case`

4. **Error Handling**
   - Use `Result` types
   - Provide descriptive error messages
   - Don't panic in library code
   ```rust
   // Good
   fn read_config() -> Result<Config, io::Error> {
       // ...
   }
   
   // Avoid
   fn read_config() -> Config {
       // ... panics on error
   }
   ```

5. **Documentation**
   - Add doc comments for public items
   ```rust
   /// Executes a tool by name with the given arguments.
   ///
   /// # Arguments
   ///
   /// * `tool_name` - Name of the tool to execute
   /// * `arguments` - JSON object with tool arguments
   ///
   /// # Returns
   ///
   /// * `Ok(String)` - Tool execution result
   /// * `Err(String)` - Error message
   pub async fn execute_tool(
       tool_name: &str,
       arguments: &serde_json::Value,
   ) -> Result<String, String>
   ```

6. **Imports**
   - Group imports logically
   - Use `use` statements, not full paths
   ```rust
   use std::fs;
   use std::io;
   
   use serde::{Deserialize, Serialize};
   use tokio::process::Command;
   
   use crate::config::Config;
   ```

---

## Submitting Changes

### 1. Push Your Branch

```bash
git push origin feature/your-feature-name
```

### 2. Create a Pull Request

1. Go to the repository on GitHub
2. Click "Pull Request"
3. Select your branch
4. Fill in the PR template:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Tests pass locally
- [ ] Added new tests
- [ ] Manual testing performed

## Checklist
- [ ] Code follows style guidelines
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] No new warnings from clippy
```

### 3. Code Review Process

- Maintainers will review your PR
- Address feedback and comments
- Make requested changes
- Push updates to the same branch

### 4. Merging

- Once approved, a maintainer will merge
- Your contribution will be in the next release!

---

## Adding Features

### Adding a New Tool

1. **Define the tool in `agent.rs`**

```rust
// Add to the match statement in execute_tool()
"my_new_tool" => {
    let arg1 = arguments["arg1"]
        .as_str()
        .ok_or("Missing arg1")?;
    
    // Tool implementation
    Ok("Success".to_string())
}
```

2. **Add tool description to system prompt**

Update the system prompt in `main.rs` to include:

```
### my_new_tool
Description: What the tool does
Arguments:
- arg1 (string): Description of argument
Returns: Description of return value
```

3. **Write tests**

Create tests in `tests/agent_tests.rs`:

```rust
#[tokio::test]
async fn test_my_new_tool() {
    let args = json!({ "arg1": "test" });
    let result = agent::execute_tool("my_new_tool", &args).await;
    assert!(result.is_ok());
}
```

4. **Update documentation**

Add to `docs/API.md`:

```markdown
##### `my_new_tool`

Description of the tool.

**Arguments:**
```json
{
    "arg1": "value"
}
```

**Returns:** Success message
```

### Adding a New Feature

1. **Discuss in an issue first**
2. **Create a design plan**
3. **Implement the feature**
4. **Add tests**
5. **Update documentation**
6. **Submit PR**

---

## Reporting Bugs

### Before Reporting

1. **Search existing issues** - Your bug may already be reported
2. **Try latest version** - Bug may be fixed
3. **Minimal reproduction** - Create simplest case that shows bug

### Bug Report Template

```markdown
**Describe the bug**
Clear description of the bug

**To Reproduce**
Steps to reproduce:
1. Launch application
2. Type command '...'
3. See error

**Expected behavior**
What you expected to happen

**Actual behavior**
What actually happened

**Environment:**
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.70]
- Application version: [e.g., 1.0.0]

**Configuration:**
```toml
[llm]
provider = "openai"
model_name = "gpt-4"
```

**Screenshots/Logs**
If applicable, add screenshots or error logs
```

---

## Development Tips

### Debugging

1. **Add logging**
   ```rust
   eprintln!("Debug: variable = {:?}", variable);
   ```

2. **Use rust-gdb or rust-lldb**
   ```bash
   rust-gdb target/debug/rust_tui_coder
   ```

3. **Check tool logs in app**
   - Tool execution details appear in tool logs area

### Testing Locally

1. **Use test config**
   ```bash
   cp config_example.toml test_config.toml
   # Add test API key
   ```

2. **Test with different models**
   - Try GPT-3.5 for faster iteration
   - Test with GPT-4 before submitting

3. **Test edge cases**
   - Empty inputs
   - Very long inputs
   - Special characters
   - Network failures

### Performance

- Profile with `cargo flamegraph`
- Check token usage efficiency
- Monitor memory with large conversations

---

## Release Process

(For maintainers)

1. Update version in `Cargo.toml`
2. Update CHANGELOG
3. Run full test suite
4. Create git tag
5. Publish to crates.io
6. Create GitHub release

---

## Getting Help

- **Questions**: Open a discussion on GitHub
- **Chat**: Join our community chat (if available)
- **Documentation**: Check the docs/ folder
- **Examples**: See EXAMPLES.md

---

## Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Acknowledged in project README

Thank you for contributing to Rust TUI Coder! 
