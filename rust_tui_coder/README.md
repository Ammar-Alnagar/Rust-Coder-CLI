# Rust TUI Coder

Rust TUI Coder is a terminal-based, interactive coding assistant powered by large language models (LLMs). It allows you to chat with an AI, write code, read files, and execute commands directly from your terminal.

## Features

- **Interactive TUI**: A user-friendly terminal interface built with `ratatui`.
- **LLM Integration**: Connects to any OpenAI-compatible API to provide intelligent responses.
- **Extensible Agent**: The agent can be extended with various tools to perform a wide range of tasks.
- **File System Operations**: Create, read, and write files.
- **Command Execution**: Run shell commands and see the output.
- **Configurable**: Easily configure the LLM provider, API key, and model through a simple TOML file.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/rust-tui-coder.git
   cd rust_tui_coder
   ```

2. **Configure your API key:**
   - Open `config.toml` and add your LLM API key and other settings:
     ```toml
     [llm]
     api_key = "YOUR_API_KEY_HERE"
     api_base_url = "https://api.openai.com/v1"
     model_name = "gpt-4"
     ```

3. **Build the project:**
   ```bash
   cargo build --release
   ```

4. **Run the application:**
   ```bash
   ./target/release/rust_tui_coder
   ```

## Usage

- **Normal Mode**: Type your message in the input box at the bottom and press `Enter` to send it to the agent.
- **Quit**: Press `q` to exit the application.

## Architecture

The application is built with a modular architecture, with each module having a distinct responsibility.

- `main.rs`: The entry point of the application. It initializes the TUI, loads the configuration, and runs the main application loop.
- `app.rs`: Manages the application's state, including user input, conversation history, and status messages.
- `ui.rs`: Renders the user interface using `ratatui`.
- `config.rs`: Loads and manages the application's configuration from `config.toml`.
- `llm.rs`: Handles all communication with the LLM API.
- `agent.rs`: Contains the core logic of the agent, including tool definitions and execution.

### Architectural Diagram

```
+-----------------+      +-----------------+      +-----------------+
|     main.rs     |----->|      app.rs     |<-----|      ui.rs      |
+-----------------+      +-----------------+      +-----------------+
        |                      ^
        |                      |
        v                      |
+-----------------+      +-----------------+
|    config.rs    |      |     agent.rs    |
+-----------------+      +-----------------+
                               |
                               |
                               v
                         +-----------------+
                         |      llm.rs     |
                         +-----------------+
```

## Future Work

- **Tool Parsing**: Implement robust parsing of the LLM's output to enable the agent to use tools like `ReadFile`, `WriteFile`, and `RunCommand`.
- **Internet Search**: Add a tool for searching the internet.
- **More UI Features**:
    - Scrolling in the conversation window.
    - Better text input components.
- **Local LLM Support**: Add support for running local LLMs.
