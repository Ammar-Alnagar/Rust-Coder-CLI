# System Architecture

## Overview

Rust TUI Coder is a terminal-based AI coding assistant built with Rust. It provides an interactive interface for developers to interact with Large Language Models (LLMs) to assist with coding tasks, file operations, and project management.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                         User Interface                       │
│                          (ui.rs)                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │ Conversation │  │  Tool Logs   │  │ Status Bar   │     │
│  │   Display    │  │   Display    │  │              │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└─────────────────────────────────────────────────────────────┘
                            ▲ │
                            │ │
                            │ ▼
┌─────────────────────────────────────────────────────────────┐
│                    Application State                         │
│                         (app.rs)                            │
│  • User input buffer                                        │
│  • Conversation history                                     │
│  • Tool execution logs                                      │
│  • Usage tracking (tokens, requests, tools)                 │
│  • Scroll state management                                  │
│  • Streaming state                                          │
└─────────────────────────────────────────────────────────────┘
                            ▲ │
                            │ │
                            │ ▼
┌─────────────────────────────────────────────────────────────┐
│                      LLM Interface                          │
│                        (llm.rs)                             │
│  • API communication                                        │
│  • Message formatting                                       │
│  • Token counting                                           │
│  • Streaming support                                        │
│  • Tool call parsing                                        │
└─────────────────────────────────────────────────────────────┘
                            ▲ │
                            │ │
                            │ ▼
┌─────────────────────────────────────────────────────────────┐
│                      Agent System                           │
│                       (agent.rs)                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │ File Ops     │  │  Code Exec   │  │  Plan Mgmt   │     │
│  │ • read       │  │  • python    │  │  • create    │     │
│  │ • write      │  │  • bash      │  │  • update    │     │
│  │ • append     │  │  • node      │  │  • clear     │     │
│  │ • search     │  │  • ruby      │  │              │     │
│  │ • delete     │  │              │  │              │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
│  ┌──────────────┐  ┌──────────────┐                        │
│  │ Directory    │  │  Git Ops     │                        │
│  │ • create     │  │  • status    │                        │
│  │ • list       │  │              │                        │
│  │ • recurse    │  │              │                        │
│  └──────────────┘  └──────────────┘                        │
└─────────────────────────────────────────────────────────────┘
                            ▲ │
                            │ │
                            │ ▼
┌─────────────────────────────────────────────────────────────┐
│                    Configuration                            │
│                       (config.rs)                           │
│  • LLM settings (API key, base URL, model)                  │
│  • Provider selection (OpenAI, Anthropic, Local)           │
│  • TOML file parsing                                        │
└─────────────────────────────────────────────────────────────┘
```

## Component Details

### 1. Main Entry Point (`main.rs`)

**Responsibilities:**
- Initialize the terminal UI using `ratatui`
- Load configuration from `config.toml`
- Set up the main event loop
- Handle user input (keyboard events)
- Coordinate between UI, App state, and LLM

**Key Functions:**
- `main()`: Entry point, sets up terminal and runs event loop
- `run_app()`: Main application loop
- Event handling for keyboard input

**Dependencies:**
- `crossterm` for terminal manipulation
- `ratatui` for TUI rendering
- `tokio` for async runtime

### 2. Application State (`app.rs`)

**Responsibilities:**
- Maintain application state
- Track conversation history
- Manage tool execution logs
- Track usage statistics (tokens, requests, tools)
- Handle scroll positions
- Manage streaming state

**Key Structures:**
```rust
pub struct App {
    pub user_input: String,
    pub conversation: Vec<String>,
    pub status_message: String,
    pub tool_logs: Vec<String>,
    pub is_executing_tool: bool,
    pub current_tool: String,
    // Usage tracking
    pub session_start_time: Instant,
    pub tokens_used: u64,
    pub total_requests: u64,
    pub total_tools_executed: u64,
    // Scrolling and streaming
    pub conversation_scroll_position: usize,
    pub tool_logs_scroll_position: usize,
    pub is_streaming: bool,
    pub current_streaming_message: String,
}
```

**Key Methods:**
- `new()`: Initialize app with default state
- `add_tool_log()`: Add tool execution logs
- `increment_*()`: Track usage metrics
- `scroll_*()`: Manage scroll positions
- `start_streaming()`, `update_streaming_message()`, `finish_streaming()`: Handle streaming responses
- `get_usage_summary()`: Generate usage statistics

### 3. User Interface (`ui.rs`)

**Responsibilities:**
- Render the terminal UI using `ratatui`
- Display conversation history
- Display tool execution logs
- Show status messages and input area
- Handle scroll rendering

**Layout:**
```
┌─────────────────────────────────────────┐
│ Conversation Area (70% height)         │
│ • User messages                         │
│ • Agent responses                       │
│ • Scrollable with UpDown                    │
└─────────────────────────────────────────┘
┌─────────────────────────────────────────┐
│ Tool Logs Area (20% height)            │
│ • Tool execution details                │
│ • Results and errors                    │
└─────────────────────────────────────────┘
┌─────────────────────────────────────────┐
│ Status Bar (1 line)                     │
│ • Commands and shortcuts                │
└─────────────────────────────────────────┘
┌─────────────────────────────────────────┐
│ Input Area (remaining)                  │
│ • User input with cursor                │
└─────────────────────────────────────────┘
```

**Key Functions:**
- `ui()`: Main rendering function
- Renders blocks with `Block::default().borders(Borders::ALL)`
- Uses `Paragraph` widgets for text display
- Implements scrolling with `scroll()` method

### 4. LLM Interface (`llm.rs`)

**Responsibilities:**
- Communicate with LLM APIs (OpenAI, Anthropic, local)
- Format messages for API requests
- Parse API responses
- Handle tool calls from LLM
- Count tokens for usage tracking
- Support streaming responses

**Key Structures:**
```rust
pub struct Message {
    pub role: String,
    pub content: String,
}

pub struct LlmResponse {
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
    pub tokens_used: u64,
}
```

**Key Functions:**
- `send_message()`: Send message to LLM and get response
- `send_message_streaming()`: Send message with streaming support
- `estimate_tokens()`: Estimate token count for text
- Format requests for different providers (OpenAI format)

**API Support:**
- OpenAI (GPT-3.5, GPT-4)
- Anthropic (Claude models)
- Local models (via OpenAI-compatible API)

### 5. Agent System (`agent.rs`)

**Responsibilities:**
- Execute tool calls requested by LLM
- Provide file system operations
- Execute code in various languages
- Manage project plans
- Handle git operations

**Tool Categories:**

#### File Operations
- `read_file`: Read file contents
- `write_file`: Write content to file
- `append_to_file`: Append content to file
- `search_and_replace`: Search and replace in file
- `delete_file`: Delete a file

#### Directory Operations
- `create_directory`: Create directory (with parents)
- `list_directory`: List directory contents
- `list_directory_recursive`: List directory tree

#### Code Execution
- `execute_python`: Run Python code
- `execute_bash`: Run bash commands
- `execute_node`: Run Node.js code
- `execute_ruby`: Run Ruby code

#### Plan Management
- `create_plan`: Create implementation plan
- `update_plan_step`: Update plan step status
- `clear_plan`: Clear the plan

#### Git Operations
- `git_status`: Get git repository status

**Key Functions:**
- `execute_tool()`: Main dispatcher for tool execution
- Individual tool implementation functions
- Error handling and result formatting

### 6. Configuration (`config.rs`)

**Responsibilities:**
- Load configuration from TOML file
- Parse LLM settings
- Provide configuration to other modules

**Configuration Structure:**
```toml
[llm]
provider = "openai"  # optional: openai, anthropic, local
api_key = "your-api-key"
api_base_url = "https://api.openai.com/v1"
model_name = "gpt-4"
```

**Key Functions:**
- `Config::from_file()`: Load config from file path
- Error handling for missing/invalid config

## Data Flow

### User Query Flow

1. User types message in input area
2. Press Enter to submit
3. `main.rs` receives input, adds to conversation
4. Message sent to LLM via `llm.rs`
5. LLM response received (may include tool calls)
6. If tool calls present:
   - Each tool call executed via `agent.rs`
   - Results logged to tool logs
   - Results sent back to LLM
   - LLM generates final response
7. Response added to conversation
8. UI updated to show new messages
9. Usage statistics updated

### Tool Execution Flow

1. LLM returns tool calls in response
2. `main.rs` parses tool calls
3. For each tool:
   - Log "Executing tool: {name}" to tool logs
   - Call `agent::execute_tool()` with tool name and arguments
   - Capture result or error
   - Log result/error to tool logs
4. Format tool results for LLM
5. Send tool results back to LLM
6. Receive and display final LLM response

### Streaming Flow

1. User submits message
2. `send_message_streaming()` called
3. App enters streaming state
4. For each chunk received:
   - Update `current_streaming_message`
   - Render UI to show partial message
5. When complete:
   - Add final message to conversation
   - Exit streaming state
   - Update scroll position

## Key Design Decisions

### 1. TUI Framework: Ratatui
- Modern, actively maintained TUI library
- Good performance and flexibility
- Widget-based architecture

### 2. Async Runtime: Tokio
- Efficient async I/O for API calls
- Non-blocking tool execution
- Streaming support

### 3. Configuration: TOML
- Human-readable format
- Easy to edit
- Strong typing with serde

### 4. Tool System: JSON-based
- LLM-friendly format
- Easy to parse and validate
- Extensible design

### 5. State Management
- Centralized in `App` struct
- Immutable where possible
- Clear ownership boundaries

## Error Handling

- File operations: Return descriptive errors
- API calls: Handle network errors, timeouts
- Tool execution: Capture and log errors
- Configuration: Fail fast with clear messages

## Performance Considerations

- Token estimation: O(n) character-based approximation
- Conversation history: Stored in memory (consider limits for long sessions)
- Tool execution: Synchronous but with progress indication
- UI rendering: Only on state changes
- Scrolling: Efficient with view windows

## Security Considerations

- API keys: Stored in config file (should not be committed)
- Code execution: Direct shell access (use in trusted environments)
- File operations: No sandboxing (user responsibility)
- Input validation: Basic validation on tool arguments

## Extension Points

### Adding New Tools
1. Define tool schema in `agent.rs`
2. Implement tool function
3. Add to `execute_tool()` dispatcher
4. Document in system prompt

### Supporting New LLM Providers
1. Add provider-specific formatting in `llm.rs`
2. Update configuration schema
3. Test with provider API

### Custom UI Themes
1. Modify `ui.rs` styling
2. Add color schemes
3. Support configuration options

## Testing Strategy

- Unit tests: Individual functions and modules
- Integration tests: Component interactions
- Performance tests: Large inputs and long sessions
- Edge case tests: Error conditions and boundaries

See [TESTING.md](TESTING.md) for detailed test documentation.

## Build and Deployment

- Build: `cargo build --release`
- Test: `cargo test`
- Lint: `cargo clippy`
- Package: `cargo package`
- Publish: `cargo publish`

See [PUBLISH.md](PUBLISH.md) for publishing instructions.
