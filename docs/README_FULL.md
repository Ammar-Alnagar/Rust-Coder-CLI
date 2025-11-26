# Rust TUI Coder - Complete Documentation

A powerful terminal-based AI coding assistant that brings AI intelligence directly to your command line. Built with Rust for performance and reliability.

## Table of Contents

1. [Overview](#overview)
2. [Features](#features)
3. [Installation](#installation)
4. [Configuration](#configuration)
5. [Usage](#usage)
6. [Available Tools](#available-tools)
7. [ReAct Pattern](#react-pattern)
8. [OS Compatibility](#os-compatibility)
9. [Custom Prompts](#custom-prompts)
10. [Examples](#examples)
11. [Troubleshooting](#troubleshooting)
12. [Contributing](#contributing)
13. [License](#license)

## Overview

Rust TUI Coder is an intelligent terminal-based coding assistant that leverages large language models (LLMs) to help you with software development tasks. It features a beautiful terminal user interface built with Ratatui and provides real-time streaming responses.

### Key Highlights

-  **AI-Powered**: Works with OpenAI, Anthropic, or local LLM providers
-  **Beautiful TUI**: Modern terminal interface with real-time updates
-  **27+ Tools**: Comprehensive toolset for development tasks
-  **ReAct Pattern**: Intelligent reasoning before acting
-  **Cross-Platform**: Works on Linux, macOS, and Windows
-  **Performance**: Built with Rust for speed and reliability
-  **Custom Prompts**: Optional prompt.md for personalized instructions

## Features

### Core Features

- **Natural Language Interface**: Communicate with the AI in plain English
- **Direct Tool Execution**: AI can manipulate files and run commands
- **Real-time Monitoring**: Watch tool execution happen live
- **Streaming Responses**: See AI responses as they're generated
- **Session Tracking**: Monitor tokens used, requests made, and tools executed
- **Planning System**: Break down complex tasks into manageable steps

### Advanced Features

- **ReAct Pattern**: AI reasons through problems before taking action
- **OS Detection**: Automatically adapts to Windows, Linux, or macOS
- **Time Awareness**: Access to current date and time
- **Enhanced File Operations**: Copy, move, and rename files easily
- **Custom System Prompts**: Add your own instructions via prompt.md
- **Multi-Language Support**: Execute code in Python, JavaScript, Rust, Go, Java, C/C++

## Installation

### From Crates.io

```bash
cargo install rust_tui_coder
```

### From Source

```bash
git clone https://github.com/Ammar-Alnagar/Rust-Coder-CLI.git
cd Rust-Coder-CLI
cargo build --release
./target/release/rust_tui_coder
```

## Configuration

Create a `config.toml` file in your working directory:

```toml
[llm]
provider = "openai"
api_key = "your-api-key-here"
api_base_url = "https://api.openai.com/v1"
model_name = "gpt-4"

# Optional: Automation settings
max_attempts = 12
workspace_root = ""
shell = "bash"
post_write_verify = true
safe_fs = true
```

### Configuration Options

- **provider**: LLM provider (openai, anthropic, or local)
- **api_key**: Your API key for the provider
- **api_base_url**: API endpoint URL
- **model_name**: Model to use (or "AUTODETECT" for automatic detection)
- **max_attempts**: Maximum tool execution attempts per task
- **workspace_root**: Root directory for file operations (empty = current directory)
- **shell**: Default shell to use (bash, sh, cmd, powershell)
- **post_write_verify**: Verify file writes
- **safe_fs**: Enable filesystem safety checks

## Usage

### Starting the Application

```bash
rust_tui_coder
```

### Basic Commands

- **Type your request** and press Enter to interact with the AI
- **/quit** - Exit the application (shows usage summary)
- **/stats** - Display current session statistics
- **UpDown** - Scroll through conversation
- **PgUp/PgDn** - Page up/down
- **Home/End** - Jump to top/bottom

### Example Interactions

```
You: Create a Python script that calculates fibonacci numbers and run it

AI: I'll create and execute the script...
[Creates fibonacci.py]
[Executes the script]
[Shows output: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34...]
```

## Available Tools

### Planning and Task Management
1. **CREATE_PLAN** - Create a structured plan for complex tasks
2. **UPDATE_PLAN** - Mark steps as completed
3. **CLEAR_PLAN** - Remove plan when task is done

### File Operations
4. **READ_FILE** - Read file contents
5. **WRITE_FILE** - Create or overwrite files
6. **APPEND_FILE** - Add content to existing files
7. **SEARCH_REPLACE** - Replace text in files
8. **DELETE_FILE** - Remove files or directories

### Enhanced File Operations
25. **COPY_FILE** - Copy files
26. **MOVE_FILE** - Move or relocate files
27. **RENAME_FILE** - Rename files or directories

### Directory Operations
9. **LIST_FILES** - List directory contents
10. **LIST_FILES_RECURSIVE** - Recursively list all files
11. **CREATE_DIRECTORY** - Create directories

### Search & Navigation
12. **GREP_SEARCH** - Search for text patterns
13. **GLOB_SEARCH** - Find files matching patterns

### Code Execution
14. **EXECUTE_CODE** - Execute code in multiple languages
15. **RUN_COMMAND** - Execute shell commands

### Development Workflow
16. **GIT_STATUS** - Show git status
17. **GIT_DIFF** - Show changes
18. **GIT_COMMIT** - Commit changes
19. **GIT_LOG** - View commit history

### Quality Assurance
20. **RUN_LINT** - Run code linters
21. **RUN_TESTS** - Execute test suites

### Package Management
22. **INSTALL_PACKAGE** - Install packages

### System Information
23. **GET_TIME** - Get current date and time
24. **GET_OS_INFO** - Get OS details and configuration

## ReAct Pattern

The AI follows the ReAct (Reasoning + Acting) pattern for better decision-making:

### How It Works

1. **REASON**: AI thinks through what needs to be done
2. **ACT**: Executes the appropriate tool
3. **OBSERVE**: Analyzes results and decides next steps

### Example

```
REASONING: I need to understand the project structure first.
ACTION: LIST_FILES in current directory
OBSERVATION: Found src/ directory with main.rs
REASONING: Now I'll read main.rs to understand the code
ACTION: READ_FILE src/main.rs
```

## OS Compatibility

The assistant automatically detects your operating system and adapts:

### OS Detection

- **Linux**: Uses bash/sh, forward slashes, Unix commands
- **macOS**: Similar to Linux with macOS-specific tools
- **Windows**: Uses cmd.exe/PowerShell, backslashes, Windows commands

### Check Your OS

```
You: What OS am I running?

AI: [Uses GET_OS_INFO tool]
Operating System Information:
• OS: Linux (Ubuntu 22.04)
• Architecture: x86_64
• Shell: bash / sh
• Path Separator: /
```

## Custom Prompts

Create a `prompt.md` file in your working directory to add custom instructions:

```markdown
# My Custom Instructions

## Code Style
- Always use meaningful variable names
- Add comments for complex logic
- Follow PEP 8 for Python code

## Project Preferences
- Use TypeScript over JavaScript
- Prefer functional programming patterns
- Write tests for all new features

## Communication Style
- Keep explanations concise
- Show code examples
- Explain your reasoning
```

The AI will automatically load and follow these instructions!

## Examples

See [EXAMPLES.md](EXAMPLES.md) for 30+ detailed usage examples including:

- File manipulation
- Code generation and execution
- Git workflows
- Project setup
- Testing and linting
- Complex multi-step tasks

## Troubleshooting

### Common Issues

**Config not found**
```bash
# Create default config
rust_tui_coder
# Follow the prompts to create config.toml
```

**API errors**
- Check your API key is correct
- Verify the API endpoint URL
- Ensure you have API credits/quota

**Tool execution fails**
- Check file permissions
- Verify required tools are installed (python3, node, git, etc.)
- Check workspace_root setting

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for more details.

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development

```bash
# Clone the repository
git clone https://github.com/Ammar-Alnagar/Rust-Coder-CLI.git
cd Rust-Coder-CLI

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run

# Build release version
cargo build --release
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE](../LICENSE))
- MIT license ([LICENSE-MIT](../LICENSE-MIT))

at your option.

## Resources

- **[Documentation Index](INDEX.md)** - Complete documentation guide
- **[Architecture](ARCHITECTURE.md)** - System design and structure
- **[API Reference](API.md)** - Complete API documentation
- **[Testing](TESTING.md)** - Test suite documentation

---

**Built with  using Rust **
