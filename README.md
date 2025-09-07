<div align="center">
  <img src="rust-coder-cli-logo.png" alt="Rust TUI Coder Logo" width="200"/>

  # 🦀 Rust TUI Coder

  **A powerful terminal-based coding assistant that brings AI intelligence directly to your command line**

  [![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
  [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=for-the-badge)](https://opensource.org/licenses/Apache-2.0)
  [![Version](https://img.shields.io/badge/version-1.0.0-orange?style=for-the-badge)]()
  [![Build Status](https://img.shields.io/badge/build-passing-brightgreen?style=for-the-badge)]()

  [📖 Documentation](#-documentation) • [🚀 Quick Start](#-getting-started) • [🛠️ Features](#-features) • [🤝 Contributing](#-contributing)

</div>

---

## 🌟 What is Rust TUI Coder?

**Rust TUI Coder** is a revolutionary terminal-based AI assistant that seamlessly blends the power of modern large language models with an intuitive, keyboard-driven interface. Designed for developers who live in the terminal, it provides an interactive environment where you can:

### 🚀 Core Capabilities

- 💬 **Chat naturally** with AI about your code and projects
- 🔧 **Execute tools** to manipulate files, run commands, and manage your workspace
- 📊 **Monitor progress** with real-time tool execution logs
- 🎨 **Enjoy a beautiful TUI** built with modern Rust technologies

### 🎯 Why Choose Rust TUI Coder?

#### For Terminal Power Users
If you spend most of your development time in the terminal, Rust TUI Coder eliminates the constant context switching between your terminal workflow and web-based AI assistants. Keep your hands on the keyboard and your mind in the flow.

#### For AI-Assisted Development
Experience true AI-assisted development with real-time tool execution. Instead of copy-pasting code snippets, let the AI directly manipulate your files, run commands, and manage your project structure while you watch the progress in real-time.

#### For Privacy-Conscious Developers
Run local models for complete privacy and security. Your code and conversations never leave your machine, giving you full control over your development environment and intellectual property.

### 🔄 Workflow Transformation

**Before: Traditional AI-Assisted Development**
```
1. Write code in editor
2. Switch to web browser
3. Copy code to AI assistant
4. Get suggestions
5. Copy suggestions back
6. Switch back to editor
7. Apply changes manually
```

**After: Integrated AI Development**
```
1. Type request in Rust TUI Coder
2. AI understands context automatically
3. Tools execute in real-time
4. See results instantly
5. Continue conversation seamlessly
```

### 🌟 Key Differentiators

| Feature | Rust TUI Coder | Web AI Assistants | IDE Extensions |
|---------|----------------|-------------------|----------------|
| **Terminal Integration** | ✅ Native | ❌ Browser-based | 🟡 Partial |
| **Direct File Access** | ✅ Full filesystem | ❌ Copy/paste only | 🟡 IDE scope only |
| **Real-time Tool Execution** | ✅ Live monitoring | ❌ None | 🟡 Limited |
| **Context Preservation** | ✅ Persistent sessions | 🟡 Per conversation | 🟡 Per file |
| **Offline Capability** | ✅ Local models | ❌ Requires internet | ❌ Requires internet |
| **Keyboard Workflow** | ✅ Full keyboard | 🟡 Mouse required | 🟡 Mixed |
| **Customization** | ✅ Extensive | 🟡 Limited | 🟡 Extension scope |
| **Privacy** | ✅ Full control | ❌ Cloud storage | 🟡 Vendor dependent |

### 💡 Perfect For

- **🚀 Rapid Prototyping**: Turn ideas into working code instantly
- **🐛 Debugging**: Get AI assistance without leaving your terminal
- **📚 Learning**: Interactive coding tutorials with immediate feedback
- **🛠️ DevOps Automation**: System administration and deployment scripts
- **📝 Documentation**: Auto-generate comprehensive project documentation
- **🔄 Code Refactoring**: Bulk operations across entire codebases
- **🏗️ Project Setup**: Automated scaffolding and configuration
- **🔍 Code Analysis**: Deep insights into large codebases

### 🎨 Built for Performance

Powered by Rust's legendary performance characteristics:
- ⚡ **Lightning-fast startup** (<2 seconds)
- 🧠 **Minimal memory footprint** (50-200MB)
- 🔄 **Real-time responsiveness** (<100ms UI updates)
- 🛡️ **Memory safe** (no garbage collection pauses)
- 📦 **Single binary deployment** (no runtime dependencies)

### 🌍 Ecosystem Integration

Seamlessly integrates with your existing development ecosystem:
- **🔧 Version Control**: Git-aware file operations
- **📦 Package Managers**: Cargo, npm, pip, apt integration
- **🛠️ Build Systems**: Support for all major build tools
- **☁️ Cloud Platforms**: AWS, GCP, Azure CLI integration
- **🐳 Containerization**: Docker and Kubernetes support
- **📊 Databases**: SQL and NoSQL database operations

### 🚀 Getting Started in 60 Seconds

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/ammar-alnagar/rust-tui-coder.git
cd rust-tui-coder
cargo build --release

# Configure your API key
cp config_example.toml config.toml
# Edit config.toml with your API key

# Launch the application
./target/release/rust_tui_coder
```

That's it! You're now ready to experience AI-assisted development like never before.

## 📋 Table of Contents

- [🌟 What is Rust TUI Coder?](#-what-is-rust-tui-coder)
- [📸 Screenshots](#-screenshots)
- [✨ Key Features](#-key-features)
  - [🖥️ Terminal User Interface](#️-terminal-user-interface)
  - [🤖 AI Integration](#-ai-integration)
  - [🛠️ Tool System](#️-tool-system)
  - [⚙️ Configuration](#️-configuration)
- [🚀 Getting Started](#-getting-started)
  - [📋 Prerequisites](#-prerequisites)
  - [⚡ Quick Installation](#-quick-installation)
  - [🔧 Configuration Setup](#-configuration-setup)
- [📖 Usage Guide](#-usage-guide)
  - [🎯 Basic Operation](#-basic-operation)
  - [💡 Example Workflows](#-example-workflows)
  - [⌨️ Keyboard Controls](#️-keyboard-controls)
- [🏗️ Architecture](#️-architecture)
- [🔧 Configuration Reference](#-configuration-reference)
- [🛠️ Development](#️-development)
- [🔍 Troubleshooting](#-troubleshooting)
- [❓ FAQ](#-faq)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)
- [📈 Performance & Comparisons](#-performance--comparisons)

## 📸 Screenshots

### Interactive Conversation & File Operations
![Rust TUI Coder in action - File creation and conversation](img1.png)
*The main interface showing a conversation where the agent creates a file and executes tools. Notice the organized layout with conversation history, tool logs, input area, and status display.*

### Tool System Overview
![Rust TUI Coder - Tool system demonstration](img2.png)
*The agent demonstrating all available tools including file operations, code execution, and directory management. The tool logs panel shows real-time execution feedback.*

## ✨ Key Features

### 🖥️ Terminal User Interface
Experience a modern, keyboard-driven interface designed for power users:

| Feature | Description |
|---------|-------------|
| **🎨 Modern TUI Design** | Built with `ratatui` - responsive, beautiful, and lightning-fast |
| **📱 Multi-Panel Layout** | 4 dedicated panels: conversation, tool logs, input area, and status |
| **📜 Smart Scrolling** | Auto-scrolls to latest messages with manual control available |
| **📝 Intelligent Text Wrapping** | Preserves formatting and readability across different terminal sizes |
| **🎭 Syntax Highlighting** | Color-coded messages for users, agents, and system outputs |
| **⌨️ Keyboard-First** | Full keyboard navigation with intuitive shortcuts |

### 🤖 AI Integration
Seamlessly integrate with leading AI providers:

| Provider | Models Supported | Key Features |
|----------|------------------|--------------|
| **OpenAI** | GPT-4, GPT-3.5-Turbo | Industry-leading reasoning and code generation |
| **Anthropic** | Claude 3 (Opus, Sonnet, Haiku) | Exceptional safety and long-context understanding |
| **Local/Custom** | Any OpenAI-compatible API | Self-hosted models, custom deployments |

- **🔄 Persistent Conversations**: Maintains full context throughout your session
- **⚡ Real-time Streaming**: Watch responses appear as they're generated
- **🛡️ Robust Error Handling**: Graceful fallbacks and informative error messages
- **🎯 Model Auto-Detection**: Automatically selects optimal models when available
- **📊 Usage Tracking**: Monitor token usage, request counts, and tool executions

### 🛠️ Tool System
A comprehensive toolkit that turns natural language into executable actions:

#### 📁 File & Directory Operations
| Tool | Purpose | Use Case |
|------|---------|----------|
| `READ_FILE` | Display file contents with line numbers | Code review, debugging, documentation |
| `WRITE_FILE` | Create/modify files with auto directory creation | Code generation, configuration files |
| `APPEND_FILE` | Add content to existing files | Log files, documentation updates |
| `REPLACE_IN_FILE` | Search and replace text with regex support | Refactoring, bulk edits |
| `DELETE_FILE` | Safe file/directory removal | Cleanup, project maintenance |
| `LIST_FILES` | Browse directory contents | Project exploration, file discovery |
| `LIST_FILES_RECURSIVE` | Deep directory traversal | Finding files across large projects |
| `CREATE_DIRECTORY` | Create directory structures | Project setup, organization |
| `COPY_PATH` | Duplicate files/directories | Backup, template creation |
| `MOVE_PATH` | Relocate files/directories | Project restructuring |

#### ⚡ Code Execution & System Integration
| Tool | Purpose | Capabilities |
|------|---------|--------------|
| `EXECUTE_CODE` | Run code in multiple languages | Python, JavaScript, Rust, Go, and more |
| `RUN_COMMAND` | Execute shell commands | Build processes, testing, deployment |
| `SEARCH_TEXT` | Find text patterns with regex | Code search, debugging, analysis |

#### 🔧 Advanced Features
- **🔄 Retry Logic**: Automatic retries with configurable limits
- **✅ Post-Write Verification**: Ensures file changes are applied correctly
- **📁 Workspace Awareness**: Context-aware file operations
- **🛡️ Safe Execution**: Sandboxed command execution with output capture
- **📊 Real-time Monitoring**: Live tool execution feedback

### ⚙️ Configuration
Flexible configuration system designed for different environments:

| Feature | Benefit |
|---------|---------|
| **📄 TOML Configuration** | Human-readable, version-controllable settings |
| **🔐 Secure Credentials** | Multiple ways to manage API keys safely |
| **🌍 Environment Variables** | Easy deployment across different systems |
| **🔧 Runtime Configuration** | Change settings without restarting |
| **📋 Validation** | Automatic validation with helpful error messages |

## 🚀 Getting Started

### 📋 Prerequisites

Before installing Rust TUI Coder, ensure your system meets these requirements:

#### System Requirements
| Component | Minimum | Recommended | Notes |
|-----------|---------|-------------|-------|
| **Operating System** | Linux, macOS, Windows | Linux/macOS | Full TUI support on Unix-like systems |
| **CPU** | 1 GHz dual-core | 2+ GHz quad-core | For responsive UI interactions |
| **RAM** | 2 GB | 4 GB+ | LLM responses require memory |
| **Storage** | 100 MB | 500 MB | For dependencies and cached data |
| **Terminal** | Modern Unicode terminal | Alacritty, iTerm2, Windows Terminal | Full color and Unicode support required |

#### Software Dependencies
| Dependency | Version | Installation | Purpose |
|------------|---------|--------------|---------|
| **Rust** | 1.70+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` | Core language and toolchain |
| **Cargo** | Latest | Included with Rust | Package manager and build tool |
| **Git** | 2.0+ | `apt/yum/brew install git` | Repository cloning |

#### LLM Provider Access
Choose one of the supported AI providers:

| Provider | Setup Required | Cost | Best For |
|----------|----------------|------|----------|
| **OpenAI** | API key + credits | Paid | Production use, reliability |
| **Anthropic** | API key + credits | Paid | Safety, long contexts |
| **Local Models** | Ollama/Local server | Free | Privacy, offline use |

### ⚡ Quick Installation

#### Option 1: Direct Download (Recommended)
```bash
# Download and extract the latest release
curl -L https://github.com/ammar-alnagar/rust-tui-coder/releases/latest/download/rust-tui-coder.tar.gz | tar xz
cd rust-tui-coder

# Make executable and run setup
chmod +x rust-tui-coder
./rust-tui-coder --setup
```

#### Option 2: Build from Source
```bash
# Clone the repository
git clone https://github.com/ammar-alnagar/rust-tui-coder.git
cd rust-tui-coder

# Build optimized release version
cargo build --release

# Optional: Install globally
cargo install --path .
```

#### Option 3: Development Setup
```bash
# Clone with submodules (if any)
git clone --recursive https://github.com/ammar-alnagar/rust-tui-coder.git
cd rust-tui-coder

# Run in development mode
cargo run
```

### 🔧 Configuration Setup

#### 1. Initial Configuration
```bash
# Copy the example configuration
cp config_example.toml config.toml

# Edit with your preferred editor
nano config.toml  # or vim, code, etc.
```

#### 2. LLM Provider Configuration

**For OpenAI:**
```toml
[llm]
api_key = "sk-your-openai-api-key-here"
api_base_url = "https://api.openai.com/v1"
model_name = "gpt-4-turbo-preview"  # or gpt-3.5-turbo
```

**For Anthropic:**
```toml
[llm]
api_key = "sk-ant-your-anthropic-key-here"
api_base_url = "https://api.anthropic.com/v1"
model_name = "claude-3-sonnet-20240229"
```

**For Local/Ollama:**
```toml
[llm]
api_key = "not-needed"
api_base_url = "http://localhost:11434/v1"
model_name = "codellama:34b"  # or any local model
```

#### 3. Advanced Configuration
```toml
[llm]
# Basic settings
api_key = "your-api-key"
api_base_url = "https://api.openai.com/v1"
model_name = "gpt-4"

# Performance tuning
max_tokens = 4000
temperature = 0.7
timeout_seconds = 30

# Tool system settings
max_attempts = 12
workspace_root = "/path/to/your/projects"
shell = "zsh"  # or bash, fish
post_write_verify = true

# UI preferences
theme = "dark"
font_size = 12
```

#### 4. Environment Variables (Alternative)
```bash
# Set environment variables instead of config file
export LLM_API_KEY="sk-your-api-key"
export LLM_API_BASE_URL="https://api.openai.com/v1"
export LLM_MODEL_NAME="gpt-4"
export LLM_MAX_TOKENS="4000"
```

### 🧪 Verification
```bash
# Test your installation
cargo run -- --version

# Test LLM connection
cargo run -- --test-connection

# Launch the application
cargo run --release
```

### 🎯 First Run Experience
1. **Launch**: `cargo run --release`
2. **Welcome Screen**: Review the welcome message
3. **First Command**: Try `/help` to see available commands
4. **Sample Interaction**: Type "Hello! Can you help me with Rust development?"
5. **Tool Discovery**: Ask "What tools do you have available?"

> 💡 **Pro Tip**: Start with simple requests like file reading or basic questions to get familiar with the interface.

## 📖 Usage Guide

### 🎯 Basic Operation

#### Interface Overview
```
┌─────────────────────────────────────────────────┐
│ Conversation Panel                              │
│ User: Hello!                                    │
│ Agent: Hi there! How can I help you today?      │
├─────────────────────────────────────────────────┤
│ Tool Logs Panel                                 │
│ ✓ Executing LIST_FILES                          │
│ ✓ Success: Found 15 files                       │
├─────────────────────────────────────────────────┤
│ Input Panel                                     │
│ > Type your message here...                     │
├─────────────────────────────────────────────────┤
│ Status Panel                                    │
│ Ready | Tokens: 1,247 | Tools: 3                │
└─────────────────────────────────────────────────┘
```

#### Core Commands
| Command | Description | Example |
|---------|-------------|---------|
| **Regular Message** | Chat naturally with AI | "Help me debug this Rust code" |
| **`/quit`** | Exit gracefully with summary | `/quit` |
| **`/stats`** | Show usage statistics | `/stats` |
| **`/help`** | Display available commands | `/help` |
| **`/clear`** | Clear conversation history | `/clear` |

### 💡 Example Workflows

#### 🌟 Getting Started Examples

**1. Simple File Operations**
```
You: Can you create a simple Rust "Hello World" program?

Agent: I'd be happy to create a Rust Hello World program for you! I'll create a new file called `hello.rs` with a basic Rust program.

[Tool Logs: ✓ WRITE_FILE executed successfully]
       ✓ File created: hello.rs (23 bytes)

Agent: Perfect! I've created `hello.rs` with a simple Hello World program. Would you like me to:
1. Show you the contents of the file?
2. Compile and run it for you?
3. Explain what each part of the code does?
```

**2. Code Analysis & Debugging**
```
You: I have a bug in my Rust code. Can you help me find it?

Agent: I'd be happy to help you debug your Rust code! Could you please:
1. Share the file path to your Rust code
2. Describe what the code is supposed to do
3. Tell me what error or unexpected behavior you're seeing

You: The file is src/main.rs and I'm getting a compilation error

Agent: Let me read your main.rs file to see what's causing the compilation error.

[Tool Logs: ✓ READ_FILE executed successfully]
       ✓ Reading: src/main.rs (156 lines)

Agent: I can see the issue! You're trying to use `unwrap()` on a `Result` that might be an error. This is on line 23. Here's what I recommend...
```

#### 🚀 Advanced Workflows

**3. Project Setup Automation**
```
You: Set up a new Rust web project with Axum and SQLx

Agent: I'll help you set up a complete Rust web project with Axum and SQLx. This will involve:

1. Creating the project structure
2. Setting up Cargo.toml with dependencies
3. Creating basic application files
4. Setting up database configuration

Let me start by creating the project structure...

[Tool Logs: ✓ CREATE_DIRECTORY executed successfully]
       ✓ Created: src/, migrations/, templates/
       ✓ WRITE_FILE executed successfully]
       ✓ Created: Cargo.toml with Axum, SQLx, Tokio
       ✓ Created: src/main.rs with basic web server
       ✓ Created: src/database.rs for DB connections

Agent: Great! I've set up your Rust web project structure. Here's what I've created:

📁 Project Structure:
├── Cargo.toml (Axum, SQLx, Tokio dependencies)
├── src/
│   ├── main.rs (Basic Axum server setup)
│   └── database.rs (SQLx connection handling)
└── migrations/ (For database schema changes)

Would you like me to:
1. Add more routes to your web server?
2. Set up authentication?
3. Create database models?
```

**4. Code Refactoring**
```
You: Help me refactor this Python script to use classes instead of functions

Agent: I'll help you refactor your Python script to use object-oriented principles. First, let me read the current file to understand its structure.

[Tool Logs: ✓ READ_FILE executed successfully]
       ✓ Reading: script.py (89 lines)

Agent: I can see your script has several functions that would benefit from being organized into classes. Here's my refactoring plan:

1. Create a `DataProcessor` class for data operations
2. Create a `FileHandler` class for file I/O
3. Create a `ReportGenerator` class for output formatting

Let me implement this refactoring...

[Tool Logs: ✓ REPLACE_IN_FILE executed successfully]
       ✓ Refactored: script.py (112 lines)
       ✓ Added classes: DataProcessor, FileHandler, ReportGenerator

Agent: Perfect! I've successfully refactored your Python script into a well-organized object-oriented structure. The new code is:

- More maintainable and extensible
- Better separation of concerns
- Easier to test individual components
- Follows Python best practices

Would you like me to explain the changes or help you write unit tests for the new classes?
```

**5. System Administration**
```
You: Set up a development environment on this Linux server

Agent: I'll help you set up a comprehensive development environment. Let me check what's already installed and then install the necessary tools.

[Tool Logs: ✓ RUN_COMMAND executed successfully]
       ✓ Checked: Current system (Ubuntu 22.04)
       ✓ RUN_COMMAND executed successfully]
       ✓ Installed: curl, wget, git, vim, htop

Agent: Good! I've installed the basic development tools. Now let me set up:

1. Node.js and npm for JavaScript development
2. Python development environment
3. Docker for containerization
4. Rust toolchain
5. Development directories and configurations

[Tool Logs: ✓ RUN_COMMAND executed successfully]
       ✓ Installed: Node.js 18.x, Python 3.10, Docker
       ✓ Installed: Rust 1.70+ stable toolchain

Agent: Excellent! Your development environment is now set up with:

✅ Node.js 18.x + npm
✅ Python 3.10 + pip
✅ Docker + docker-compose
✅ Rust 1.70+ toolchain
✅ Essential development tools (git, vim, htop, curl)

Would you like me to:
1. Set up a sample project to test everything?
2. Configure your shell environment?
3. Install additional development tools?
```

#### 🔧 Specialized Use Cases

**6. Documentation Generation**
```
You: Generate comprehensive documentation for my Rust project

Agent: I'll create thorough documentation for your Rust project by:
1. Reading the source code structure
2. Generating API documentation
3. Creating usage examples
4. Writing a README with setup instructions

Let me start by exploring your project structure...

[Tool Logs: ✓ LIST_FILES_RECURSIVE executed successfully]
       ✓ Found: 23 Rust files, 5 config files
       ✓ READ_FILE executed successfully]
       ✓ Analyzed: src/lib.rs, src/main.rs

Agent: Based on my analysis, I'll create:
1. Comprehensive README.md
2. API documentation comments
3. Usage examples
4. Contributing guidelines

[Tool Logs: ✓ WRITE_FILE executed successfully]
       ✓ Created: README.md (450 lines)
       ✓ WRITE_FILE executed successfully]
       ✓ Created: docs/API.md (320 lines)
       ✓ WRITE_FILE executed successfully]
       ✓ Created: examples/ (with 5 example files)

Agent: Documentation generation complete! I've created:
- 📖 Detailed README with setup and usage
- 📚 API documentation for all public functions
- 💡 5 practical examples with explanations
- 🤝 Contributing guidelines for the project
```

### ⌨️ Keyboard Controls

#### Basic Navigation
| Key | Action | Context |
|-----|--------|---------|
| **Enter** | Send message | Input panel |
| **Backspace** | Delete character | Input panel |
| **Tab** | Auto-complete | Input panel |
| **↑/↓** | Navigate history | Input panel |

#### Advanced Controls
| Key Combination | Action | Description |
|-----------------|--------|-------------|
| **Ctrl+C** | Emergency exit | Force quit application |
| **Ctrl+L** | Clear screen | Refresh display |
| **Ctrl+R** | Search history | Find previous messages |
| **Ctrl+Z** | Suspend | Background process (Unix only) |

#### Special Commands
| Command | Purpose | Example |
|---------|---------|---------|
| `/quit` | Graceful exit with summary | `/quit` |
| `/stats` | Show usage statistics | `/stats` |
| `/clear` | Clear conversation | `/clear` |
| `/help` | Show available commands | `/help` |
| `/save` | Save conversation to file | `/save discussion.txt` |
| `/load` | Load conversation from file | `/load previous_session.txt` |

### 🎨 Interface Customization

#### Color Themes
```bash
# Set theme via environment variable
export RUST_TUI_CODER_THEME="dark"  # or "light", "auto"

# Or via config file
[ui]
theme = "dark"
syntax_highlighting = true
```

#### Layout Options
```toml
[ui]
# Panel proportions (must sum to 100)
conversation_panel = 60
tool_logs_panel = 20
input_panel = 10
status_panel = 10

# Text display options
word_wrap = true
show_line_numbers = true
max_display_width = 120
```

## 🏗️ Architecture

Rust TUI Coder follows a clean, modular architecture designed for maintainability and extensibility:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│    main.rs      │───▶│     app.rs      │◀───│     ui.rs       │
│                 │    │                 │    │                 │
│ • Entry point   │    │ • State mgmt    │    │ • UI rendering  │
│ • TUI setup     │    │ • Event loop    │    │ • Layout mgmt   │
│ • Config load   │    │ • User input    │    │ • Styling       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       ▲
         │                       │
         ▼                       │
┌─────────────────┐    ┌─────────────────┐
│   config.rs     │    │    agent.rs     │
│                 │    │                 │
│ • TOML parsing  │    │ • Tool system   │
│ • Settings mgmt │    │ • LLM interface │
│ • Validation    │    │ • Task exec     │
└─────────────────┘    └─────────────────┘
                                │
                                │
                                ▼
                       ┌─────────────────┐
                       │     llm.rs      │
                       │                 │
                       │ • HTTP client   │
                       │ • API requests  │
                       │ • Response parse│
                       │ • Error handling│
                       └─────────────────┘
```

### Module Responsibilities

| Module | Purpose | Key Features |
|--------|---------|--------------|
| **main.rs** | Application entry point | Terminal setup, event loop, graceful shutdown |
| **app.rs** | State management | Conversation history, user input, tool logs |
| **ui.rs** | User interface | Multi-panel layout, scrolling, text formatting |
| **config.rs** | Configuration handling | TOML parsing, validation, defaults |
| **agent.rs** | AI agent logic | Tool definitions, execution, LLM communication |
| **llm.rs** | LLM communication | HTTP requests, response parsing, error handling |

## 🔧 Configuration Reference

### Complete Configuration Example

```toml
[llm]
# Your API key for the LLM provider
api_key = "sk-..."

# Base URL for the API endpoint
api_base_url = "https://api.openai.com/v1"

# Model name to use for completions
model_name = "gpt-4"

# Optional: Additional settings (future expansion)
# max_tokens = 4000
# temperature = 0.7
# timeout_seconds = 30
```

### Environment Variables (Alternative)

You can also use environment variables instead of the config file:

```bash
export LLM_API_KEY="your-api-key"
export LLM_API_BASE_URL="https://api.openai.com/v1"
export LLM_MODEL_NAME="gpt-4"
```

## 🛠️ Development

### 🚀 Getting Started with Development

#### Development Environment Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/ammar-alnagar/rust-tui-coder.git
   cd rust-tui-coder
   ```

2. **Set up development tools:**
   ```bash
   # Install development dependencies
   cargo install cargo-watch cargo-expand cargo-udeps cargo-audit

   # Optional: Install additional tools
   cargo install cargo-tarpaulin cargo-nextest
   ```

3. **Verify your setup:**
   ```bash
   # Check Rust version
   rustc --version
   cargo --version

   # Run initial build
   cargo check

   # Run tests to ensure everything works
   cargo test
   ```

### 🏗️ Build System

#### Build Profiles

| Profile | Command | Use Case | Optimizations |
|---------|---------|----------|---------------|
| **Debug** | `cargo build` | Development | None, fast compilation |
| **Release** | `cargo build --release` | Production | Full optimizations |
| **Dev Optimized** | `cargo build --profile dev-optimized` | Fast development | Some optimizations |

#### Advanced Build Options

```bash
# Build with specific features
cargo build --features "additional-tools"

# Build for different targets
cargo build --target x86_64-unknown-linux-gnu
cargo build --target aarch64-apple-darwin

# Cross-compilation (requires cross)
cross build --target armv7-unknown-linux-gnueabihf --release
```

#### Development Workflow

```bash
# Watch mode for continuous rebuilding
cargo watch -x check

# Run tests on file changes
cargo watch -x test

# Format and lint on save
cargo watch -x 'fmt && clippy'

# Full development cycle
cargo watch -x 'check && test && fmt && clippy'
```

### 🧪 Testing

#### Test Categories

| Test Type | Command | Coverage | Speed |
|-----------|---------|----------|-------|
| **Unit Tests** | `cargo test` | Individual functions | ⚡ Fast |
| **Integration Tests** | `cargo test --test integration` | Module interactions | 🟡 Medium |
| **End-to-End Tests** | `cargo test --test e2e` | Full workflows | 🟡 Medium |
| **Benchmark Tests** | `cargo bench` | Performance | 🐌 Slow |

#### Testing Commands

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run only integration tests
cargo test --test integration

# Run tests for specific module
cargo test --package rust_tui_coder --lib agent

# Generate coverage report (requires tarpaulin)
cargo tarpaulin --out Html --output-dir coverage/

# Run benchmarks
cargo bench
```

### 🔧 Code Quality

#### Linting & Formatting

```bash
# Format code
cargo fmt

# Run clippy lints
cargo clippy

# Fix auto-fixable issues
cargo clippy --fix

# Check for unused dependencies
cargo udeps

# Security audit
cargo audit
```

### 📚 Architecture & Code Organization

#### Module Structure

```
src/
├── main.rs          # Application entry point
├── app.rs           # State management
├── ui.rs            # Terminal user interface
├── agent.rs         # AI agent and tool system
├── llm.rs           # LLM API communication
└── config.rs        # Configuration handling
```

#### Key Design Patterns

1. **State Management**
   - Central `App` struct for application state
   - Immutable updates with builder pattern
   - Event-driven architecture

2. **Error Handling**
   - Custom error types with `thiserror`
   - Proper error propagation with `anyhow`
   - User-friendly error messages

3. **Async Programming**
   - `tokio` for async runtime
   - Structured concurrency patterns
   - Proper cancellation handling

4. **Configuration Management**
   - TOML-based configuration
   - Environment variable support
   - Runtime configuration reloading

### 🛠️ Tool Development

#### Adding New Tools

1. **Define the tool function:**
   ```rust
   async fn my_custom_tool(args: HashMap<String, Value>) -> Result<String, Box<dyn Error>> {
       // Tool implementation
       let path = args.get("path").ok_or("Missing 'path' argument")?;
       // ... tool logic ...
       Ok(format!("Tool executed successfully on {}", path))
   }
   ```

2. **Register the tool:**
   ```rust
   impl Agent {
       pub fn register_tools(&mut self) {
           self.tools.insert("my_custom_tool".to_string(), my_custom_tool);
       }
   }
   ```

#### Tool Best Practices

- **Input Validation**: Always validate input parameters
- **Error Handling**: Provide clear, actionable error messages
- **Idempotency**: Tools should be safe to run multiple times
- **Logging**: Use appropriate log levels for different operations
- **Performance**: Consider async operations for I/O-bound tasks
- **Security**: Validate file paths and command arguments

### 🤝 Contributing Guidelines

#### Development Workflow

1. **Choose an issue** from the [issue tracker](../../issues)
2. **Create a feature branch:**
   ```bash
   git checkout -b feature/amazing-feature
   # or
   git checkout -b fix/bug-description
   ```
3. **Make your changes** following the established patterns
4. **Write tests** for new functionality
5. **Update documentation** if needed
6. **Run quality checks:**
   ```bash
   cargo fmt && cargo clippy && cargo test
   ```
7. **Commit your changes:**
   ```bash
   git commit -m "feat: add amazing new feature

   - What was changed
   - Why it was changed
   - How it was implemented"
   ```

#### Code Standards

**Rust Idioms:**
- Use `snake_case` for variables and functions
- Use `PascalCase` for types and traits
- Prefer `&str` over `&String` for function parameters
- Use `Result<T, E>` for error handling
- Implement `Debug` and `Clone` where appropriate

**Documentation:**
- Add doc comments to all public APIs
- Include examples in documentation
- Document error conditions and edge cases
- Keep README and docs up to date

**Testing:**
- Write tests for all new functionality
- Include both positive and negative test cases
- Test error conditions and edge cases
- Aim for >80% code coverage

### 📦 Dependencies

#### Core Dependencies

| Crate | Version | Purpose | License |
|-------|---------|---------|---------|
| `ratatui` | 0.26.0 | Terminal UI framework | MIT |
| `crossterm` | 0.27.0 | Terminal manipulation | MIT |
| `tokio` | 1.35.1 | Async runtime | MIT |
| `reqwest` | 0.11.23 | HTTP client | MIT/Apache-2.0 |
| `serde` | 1.0.195 | Serialization | MIT/Apache-2.0 |
| `serde_json` | 1.0.111 | JSON handling | MIT/Apache-2.0 |
| `toml` | 0.8.8 | TOML parsing | MIT/Apache-2.0 |
| `anyhow` | 1.0.75 | Error handling | MIT/Apache-2.0 |
| `thiserror` | 1.0.50 | Error types | MIT/Apache-2.0 |

## 🔍 Troubleshooting

### Common Issues & Solutions

#### 🚫 Build & Compilation Issues

**Issue: "error[E0658]: use of unstable library feature"**
```
error[E0658]: use of unstable library feature `async_fn_in_trait`
```
**Solution:**
```bash
# Update Rust toolchain to latest stable
rustup update stable
rustup default stable

# Or use a specific nightly version that supports the features
rustup install nightly-2024-01-01
rustup override set nightly-2024-01-01
```

**Issue: "failed to select a version for the requirement `ratatui = "^0.26.0"`"**
```
error: failed to select a version for the requirement `ratatui = "^0.26.0"`
```
**Solution:**
```bash
# Update Cargo index
cargo update

# Or clear Cargo cache and rebuild
rm -rf ~/.cargo/registry/cache/
rm -rf ~/.cargo/git/checkouts/
cargo clean
cargo build
```

#### 🔧 Runtime Issues

**Issue: "Permission denied" when running tools**
```
Error: Permission denied (os error 13)
```
**Solution:**
```bash
# Check file permissions
ls -la target/release/rust_tui_coder

# Make executable if needed
chmod +x target/release/rust_tui_coder

# Or run with explicit permissions
sudo target/release/rust_tui_coder
```

**Issue: Terminal displays garbled text or escape sequences**
```
35;102;43M35;103;43M35;103;42M...
```
**Solution:**
```toml
# In config.toml, ensure mouse capture is disabled
[ui]
mouse_capture = false
raw_mode = true
```

**Issue: "Connection refused" to LLM API**
```
Error: Connection refused (os error 111)
```
**Solution:**
```bash
# Check network connectivity
curl -I https://api.openai.com/v1/models

# Verify API key and endpoint in config
cat config.toml | grep -E "(api_key|api_base_url)"

# Test with a simple curl command
curl -H "Authorization: Bearer YOUR_API_KEY" \
     -H "Content-Type: application/json" \
     https://api.openai.com/v1/models
```

#### 🎨 Display & UI Issues

**Issue: Interface appears broken or misaligned**
```
Symptoms: Panels overlap, text wraps incorrectly, colors don't display
```
**Solution:**
```bash
# Check terminal capabilities
echo $TERM

# Force a compatible terminal type
export TERM=xterm-256color

# Test with a simpler terminal
# Or resize terminal window to at least 80x24 characters
```

**Issue: Colors don't display correctly**
```
Expected: Colored text and UI elements
Actual: Plain text only
```
**Solution:**
```bash
# Enable 256-color support
export TERM=xterm-256color

# Or force color output
export CLICOLOR_FORCE=1

# Check if terminal supports colors
tput colors
```

#### 🔑 Configuration Issues

**Issue: "No such file or directory" for config.toml**
```
Error: No such file or directory (os error 2)
```
**Solution:**
```bash
# Copy the example configuration
cp config_example.toml config.toml

# Or create minimal config manually
cat > config.toml << EOF
[llm]
api_key = "your-api-key-here"
api_base_url = "https://api.openai.com/v1"
model_name = "gpt-4"
EOF
```

**Issue: API key not recognized**
```
Error: Invalid API key or unauthorized access
```
**Solution:**
```bash
# Check API key format
echo $LLM_API_KEY | head -c 10  # Should start with sk-

# Verify key in config file
grep api_key config.toml

# Test key with direct API call
curl -H "Authorization: Bearer YOUR_API_KEY" \
     https://api.openai.com/v1/models | jq .error
```

#### 🔧 Tool Execution Issues

**Issue: Tools fail with "command not found"**
```
Error: bash: cargo: command not found
```
**Solution:**
```bash
# Check if command exists in PATH
which cargo

# Install missing tool
# For Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# For Node.js: curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
# For Python: sudo apt install python3 python3-pip

# Update PATH in shell profile
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Issue: File operations fail due to permissions**
```
Error: Permission denied when writing to file
```
**Solution:**
```bash
# Check current directory permissions
pwd && ls -la

# Change to a directory with write permissions
cd ~/projects/

# Or fix permissions on current directory
chmod 755 .
chmod 644 *.rs  # for Rust files
```

### 🐛 Debugging Commands

#### Test LLM Connection
```bash
# Test basic connectivity
curl -H "Authorization: Bearer $LLM_API_KEY" \
     -H "Content-Type: application/json" \
     "$LLM_API_BASE_URL/models"

# Test with the application
cargo run -- --test-connection
```

#### Check System Resources
```bash
# Memory usage
free -h

# Disk space
df -h

# Network connectivity
ping -c 3 api.openai.com
```

#### Generate Debug Logs
```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Or save logs to file
RUST_LOG=debug cargo run 2>&1 | tee debug.log
```

#### Reset Application State
```bash
# Clear all caches and temporary files
cargo clean
rm -rf target/
rm -f *.log

# Reset configuration
cp config_example.toml config.toml
```

### 📞 Getting Help

If you can't resolve an issue:

1. **Check the [Issues](../../issues) page** for similar problems
2. **Enable debug logging** and share relevant logs
3. **Provide system information:**
   ```bash
   uname -a
   rustc --version
   cargo --version
   echo $TERM
   ```
4. **Create a new issue** with:
   - Your operating system and version
   - Rust version (`rustc --version`)
   - Full error message and stack trace
   - Steps to reproduce the issue
   - Your `config.toml` (with API keys removed)

## ❓ FAQ

### 🤖 General Questions

**Q: What is Rust TUI Coder?**
A: Rust TUI Coder is a terminal-based AI assistant that combines the power of modern language models with an intuitive keyboard-driven interface. It allows you to chat with AI while executing tools to manipulate files, run commands, and manage your development environment—all without leaving your terminal.

**Q: How is this different from ChatGPT or GitHub Copilot?**
A: Unlike web-based AI assistants, Rust TUI Coder:
- Keeps you in your terminal workflow
- Provides real-time tool execution with visual feedback
- Offers persistent conversations and project context
- Enables direct file and system manipulation
- Works offline with local models

**Q: Is it free to use?**
A: The application itself is free and open-source. You only pay for the underlying LLM API usage (OpenAI, Anthropic, etc.) based on their pricing models.

**Q: Can I use it offline?**
A: Yes! You can use local models via Ollama or other local LLM servers. Simply configure the `api_base_url` to point to your local instance.

### 🛠️ Technical Questions

**Q: Which operating systems are supported?**
A: Linux (primary), macOS, and Windows (via WSL). Full TUI support requires a modern Unicode terminal.

**Q: Which LLM providers are supported?**
A: OpenAI (GPT-4, GPT-3.5), Anthropic (Claude), and any OpenAI-compatible API (local models, custom deployments).

**Q: Can I use my own API keys?**
A: Yes, the application supports API keys for all major providers and can read them from configuration files or environment variables.

**Q: How much RAM do I need?**
A: Minimum 2GB, recommended 4GB+. Memory usage depends on the LLM model and conversation length.

**Q: Can I customize the interface?**
A: Yes! You can customize colors, panel layouts, keyboard shortcuts, and more through the configuration file.

### 🔧 Usage Questions

**Q: How do I exit the application?**
A: Type `/quit` and press Enter for a graceful exit with usage summary, or use Ctrl+C for emergency exit.

**Q: Can I save my conversations?**
A: Yes, use `/save filename.txt` to save your conversation history to a file.

**Q: What commands are available?**
A: Type `/help` in the application to see all available commands and tools.

**Q: How do I clear the conversation?**
A: Use `/clear` to reset the conversation history while keeping the application running.

**Q: Can I load previous conversations?**
A: Yes, use `/load filename.txt` to load a previously saved conversation.

### 🐛 Troubleshooting Questions

**Q: Why do I see strange escape sequences?**
A: This usually happens when mouse capture is enabled but your terminal doesn't support it properly. Disable mouse capture in your config or use a different terminal.

**Q: The interface looks broken in my terminal**
A: Try resizing your terminal to at least 80x24 characters, or set `TERM=xterm-256color` in your environment.

**Q: Tools are failing with permission errors**
A: Check file permissions and ensure you're running the application with appropriate access to the directories you want to modify.

**Q: API connection is failing**
A: Verify your API key, check your internet connection, and ensure your firewall isn't blocking the API endpoints.

**Q: The application is slow**
A: Try using a smaller model, check your internet connection, or use a local model for better performance.

### 🛠️ Development Questions

**Q: Can I contribute to the project?**
A: Absolutely! See the Contributing section below for guidelines on how to help improve Rust TUI Coder.

**Q: How do I add new tools?**
A: Tools are defined in the agent module. Create a new tool function and add it to the tool registry following the existing patterns.

**Q: Can I modify the UI layout?**
A: Yes, the UI is built with `ratatui` and can be customized in the `ui.rs` module.

**Q: How do I add support for new LLM providers?**
A: Add a new case to the LLM client in `llm.rs` following the existing OpenAI/Anthropic patterns.

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**: Follow Rust best practices and add tests
4. **Commit your changes**: `git commit -m 'Add amazing feature'`
5. **Push to the branch**: `git push origin feature/amazing-feature`
6. **Open a Pull Request**

### Development Guidelines
- Follow Rust naming conventions and idioms
- Add tests for new functionality
- Update documentation for API changes
- Ensure `cargo clippy` passes without warnings
- Format code with `cargo fmt`

## 📈 Performance & Comparisons

### 🚀 Performance Benchmarks

#### System Requirements & Performance

| Component | Minimum | Recommended | Performance Impact |
|-----------|---------|-------------|-------------------|
| **CPU** | 1 GHz dual-core | 2+ GHz quad-core | UI responsiveness, tool execution speed |
| **RAM** | 2 GB | 4 GB+ | LLM response caching, concurrent operations |
| **Network** | 10 Mbps | 50+ Mbps | API response times, real-time streaming |
| **Storage** | 100 MB | 500 MB+ | Dependencies, cached responses |

#### Benchmark Results (Approximate)

| Operation | Local Model | OpenAI GPT-4 | Anthropic Claude |
|-----------|-------------|--------------|------------------|
| **Simple Query** | 0.5-2s | 2-5s | 2-4s |
| **Code Generation** | 1-3s | 3-8s | 3-7s |
| **File Analysis** | 0.2-1s | 1-3s | 1-3s |
| **Tool Execution** | 0.1-0.5s | 0.1-0.5s | 0.1-0.5s |
| **UI Rendering** | <0.1s | <0.1s | <0.1s |

*Benchmarks performed on Ubuntu 22.04 with 8GB RAM, 100Mbps internet*

### 🔍 Feature Comparison

#### vs. ChatGPT Web Interface

| Feature | Rust TUI Coder | ChatGPT Web |
|---------|----------------|-------------|
| **Terminal Integration** | ✅ Native | ❌ Limited |
| **File System Access** | ✅ Direct | ❌ Manual copy/paste |
| **Tool Execution** | ✅ Real-time | ❌ None |
| **Offline Capability** | ✅ Local models | ❌ Requires internet |
| **Keyboard Navigation** | ✅ Full keyboard | 🟡 Mouse required |
| **Customization** | ✅ Extensive | 🟡 Limited |
| **Privacy** | ✅ Local storage | ❌ Cloud storage |
| **Cost** | 🟡 API usage only | 🟡 Subscription |

#### vs. GitHub Copilot

| Feature | Rust TUI Coder | GitHub Copilot |
|---------|----------------|----------------|
| **Interface** | Terminal TUI | IDE integration |
| **Language Support** | All languages | IDE-supported languages |
| **Context Awareness** | Full project | Current file + tabs |
| **Tool Integration** | File system, commands | IDE features only |
| **Conversation Memory** | Persistent sessions | Per-file context |
| **Offline Usage** | ✅ Local models | ❌ Requires internet |
| **Cost** | API usage | Subscription |

#### vs. Cursor/VS Code AI Extensions

| Feature | Rust TUI Coder | Cursor/VS Code AI |
|---------|----------------|------------------|
| **Terminal Focus** | ✅ Dedicated | ❌ IDE-centric |
| **File Operations** | ✅ Direct manipulation | 🟡 Through IDE |
| **System Commands** | ✅ Full shell access | ❌ Limited |
| **Multi-file Editing** | ✅ Bulk operations | 🟡 Sequential |
| **Project Automation** | ✅ Script generation | 🟡 Manual workflows |
| **Resource Usage** | 🟡 Lightweight | 🟡 IDE overhead |

### 💰 Cost Analysis

#### API Usage Costs (Approximate monthly)

| Usage Pattern | OpenAI GPT-4 | Anthropic Claude | Local Models |
|---------------|--------------|------------------|--------------|
| **Light (10 queries/day)** | $5-15 | $3-10 | $0 (one-time setup) |
| **Moderate (50 queries/day)** | $25-75 | $15-50 | $0 |
| **Heavy (200+ queries/day)** | $100-300 | $60-200 | $0 |

#### Total Cost of Ownership

```
Hardware Investment: $0-500 (if upgrading)
Software: $0 (free and open-source)
API Usage: $10-100/month (depending on usage)
Local Models: $0-50 one-time (optional GPU upgrade)
```

### 🏆 Strengths & Advantages

#### ✅ Key Advantages

1. **🏃‍♂️ Speed & Efficiency**
   - Lightning-fast TUI rendering (<100ms)
   - Direct file system access (no copy/paste)
   - Keyboard-driven workflow (no mouse required)
   - Real-time tool execution feedback

2. **🔒 Privacy & Security**
   - Local conversation storage
   - No cloud dependency for basic features
   - API keys stored locally
   - Full control over data

3. **🔧 Developer Experience**
   - Terminal-native workflow
   - Seamless integration with existing tools
   - Extensible architecture for custom tools
   - Rust performance and reliability

4. **💰 Cost Effectiveness**
   - Free application (open-source)
   - Pay only for API usage
   - Local models eliminate API costs entirely
   - No subscription fees

#### 🎯 Use Cases & Scenarios

**Perfect For:**
- 🚀 **Rapid Prototyping**: Quick project setup and file generation
- 🐛 **Debugging**: Code analysis with immediate file access
- 📚 **Learning**: Interactive coding tutorials and explanations
- 🛠️ **DevOps**: System administration and automation
- 📝 **Documentation**: Auto-generating project docs and READMEs
- 🔄 **Refactoring**: Bulk code changes across multiple files

**Ideal Users:**
- 💻 **Terminal Power Users**: Developers who live in the command line
- 🏢 **DevOps Engineers**: System administrators and automation specialists
- 🎓 **Students**: Learning programming with interactive AI assistance
- 🚀 **Startup Developers**: Rapid prototyping and MVP development
- 🔧 **Open Source Contributors**: Working across multiple projects

### 🔬 Technical Specifications

#### System Architecture
```
┌─────────────────────────────────────┐
│         Rust TUI Coder              │
├─────────────────────────────────────┤
│ 🖥️  TUI Layer (ratatui + crossterm) │
│ 🤖 Agent Layer (tool execution)     │
│ 🌐 LLM Layer (API communication)    │
│ ⚙️  Config Layer (TOML + env vars)   │
└─────────────────────────────────────┘
```

#### Dependencies & Ecosystem
- **Core Framework**: `ratatui` for terminal UI, `crossterm` for input handling
- **Async Runtime**: `tokio` for concurrent operations
- **HTTP Client**: `reqwest` for API communication
- **Serialization**: `serde` for configuration and data handling
- **Error Handling**: Comprehensive error propagation and user feedback

#### Performance Characteristics
- **Memory Usage**: 50-200MB (depending on conversation size)
- **CPU Usage**: Minimal (<5%) during idle, spikes during LLM calls
- **Network Usage**: 10-100KB per API call (excluding streaming)
- **Disk Usage**: 100MB base + conversation logs
- **Startup Time**: <2 seconds (optimized release build)

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.



**Happy Coding! 🦀✨**

## Updates (2025-08-18)

### New Features
- **Expanded Toolset**: Added `AppendFile`, `ReplaceInFile`, `SearchText`, `CopyPath`, `MovePath`, and `ListFilesRecursive` for more powerful automation.
- **JSON Tool Calls**: Supports structured JSON format for tool calls (backward-compatible with legacy format).
- **Configurable Automation**:
  - `max_attempts`: Limit retries per task (default: 12).
  - `workspace_root`: Set a fixed working directory (default: current dir).
  - `shell`: Default shell for commands (default: `bash`).
  - `post_write_verify`: Auto-verify file changes (default: `true`).
- **Improved Prompt**: Now includes workspace context, OS info, and stricter guidance for autonomous task completion.
- **Model Auto-Detection**: When `model_name = "AUTODETECT"`, the system automatically selects the first available model from your LLM provider's API.

## Architecture

### System Overview
```mermaid
graph TD
    A[User Input] --> B(Agent)
    B --> C{Tool Call?}
    C -->|Yes| D[Execute Tool]
    D --> E[Verify Results]
    E --> B
    C -->|No| F[Final Output]
```

### Tool Execution Flow
```mermaid
sequenceDiagram
    participant User
    participant Agent
    participant Tools
    User->>Agent: Task Request
    Agent->>Tools: TOOL: {"name":"WRITE_FILE", ...}
    Tools-->>Agent: Success/Failure
    Agent->>User: Tool Logs + Next Step
```

### Task Completion Logic
```mermaid
flowchart LR
    Start --> Plan
    Plan --> Execute
    Execute --> Verify
    Verify -->|Success| Done
    Verify -->|Failure| Adjust
    Adjust --> Execute
    Done --> End
```

### Example: Adding a CLI Command
```mermaid
graph LR
    A[Task: 'Add version flag'] --> B[List Files]
    B --> C[Find main.rs]
    C --> D[Edit main.rs]
    D --> E[Compile]
    E --> F[Test]
    F --> G[Done]
```

These diagrams visualize:
1. The **end-to-end flow** from input to output
2. **Tool call mechanics** (JSON/legacy)
3. The **autonomous loop** (plan → execute → verify)
4. A **concrete example** of adding a CLI command
