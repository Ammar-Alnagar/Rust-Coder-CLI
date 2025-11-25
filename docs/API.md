# API Documentation

## Table of Contents

1. [LLM Module](#llm-module)
2. [Agent Module](#agent-module)
3. [App Module](#app-module)
4. [Config Module](#config-module)
5. [UI Module](#ui-module)

---

## LLM Module

The LLM module handles communication with Large Language Model APIs.

### Types

#### `Message`

Represents a message in the conversation.

```rust
pub struct Message {
    pub role: String,    // "user", "assistant", or "system"
    pub content: String, // Message content
}
```

**Methods:**

- `new(role: &str, content: &str) -> Self`
  - Creates a new message with the given role and content

#### `LlmResponse`

Represents a response from the LLM API.

```rust
pub struct LlmResponse {
    pub content: String,              // Response text
    pub tool_calls: Vec<ToolCall>,    // Requested tool calls
    pub tokens_used: u64,             // Tokens consumed
}
```

### Functions

#### `send_message`

```rust
pub async fn send_message(
    config: &LlmConfig,
    messages: &[Message],
) -> Result<LlmResponse, Box<dyn std::error::Error>>
```

Sends a message to the LLM API and returns the response.

**Parameters:**
- `config`: LLM configuration (API key, base URL, model)
- `messages`: Conversation history

**Returns:**
- `Ok(LlmResponse)`: Successful response with content and tool calls
- `Err(error)`: API error, network error, or parse error

**Example:**
```rust
let config = LlmConfig { /* ... */ };
let messages = vec![
    Message::new("user", "Hello!"),
];
let response = send_message(&config, &messages).await?;
println!("Response: {}", response.content);
```

#### `send_message_streaming`

```rust
pub async fn send_message_streaming(
    config: &LlmConfig,
    messages: &[Message],
    callback: impl Fn(String),
) -> Result<LlmResponse, Box<dyn std::error::Error>>
```

Sends a message with streaming support, calling the callback for each chunk.

**Parameters:**
- `config`: LLM configuration
- `messages`: Conversation history
- `callback`: Function called with each content chunk

**Returns:**
- `Ok(LlmResponse)`: Complete response after streaming
- `Err(error)`: API or network error

**Example:**
```rust
let response = send_message_streaming(&config, &messages, |chunk| {
    print!("{}", chunk);
}).await?;
```

#### `estimate_tokens`

```rust
pub fn estimate_tokens(text: &str) -> u64
```

Estimates the number of tokens in a text string.

**Parameters:**
- `text`: Text to estimate

**Returns:**
- Estimated token count (approximately text.len() / 4)

**Note:** This is a rough approximation. Actual token counts may vary by model.

#### `format_tool_results`

```rust
pub fn format_tool_results(tool_results: &[(String, String)]) -> String
```

Formats tool execution results for sending back to the LLM.

**Parameters:**
- `tool_results`: Vector of (tool_name, result) tuples

**Returns:**
- Formatted string describing tool results

---

## Agent Module

The agent module provides tool execution capabilities.

### Types

#### `ToolCall`

Represents a tool call requested by the LLM.

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}
```

**Fields:**
- `id`: Unique identifier for the tool call
- `name`: Name of the tool to execute
- `arguments`: JSON object with tool arguments

### Functions

#### `execute_tool`

```rust
pub async fn execute_tool(
    tool_name: &str,
    arguments: &serde_json::Value,
) -> Result<String, String>
```

Executes a tool by name with the given arguments.

**Parameters:**
- `tool_name`: Name of the tool to execute
- `arguments`: JSON object with tool-specific arguments

**Returns:**
- `Ok(String)`: Tool execution result
- `Err(String)`: Error message

**Example:**
```rust
let args = json!({
    "path": "example.txt"
});
let result = execute_tool("read_file", &args).await?;
```

### Available Tools

#### File Operations

##### `read_file`

Read the contents of a file.

**Arguments:**
```json
{
    "path": "file/path.txt"
}
```

**Returns:** File contents as string

---

##### `write_file`

Write content to a file (overwrites existing content).

**Arguments:**
```json
{
    "path": "file/path.txt",
    "content": "Content to write"
}
```

**Returns:** Success message

---

##### `append_to_file`

Append content to the end of a file.

**Arguments:**
```json
{
    "path": "file/path.txt",
    "content": "Content to append"
}
```

**Returns:** Success message

---

##### `search_and_replace`

Search for text in a file and replace it.

**Arguments:**
```json
{
    "path": "file/path.txt",
    "search": "text to find",
    "replace": "replacement text"
}
```

**Returns:** Success message with replacement count

---

##### `delete_file`

Delete a file or directory.

**Arguments:**
```json
{
    "path": "file/path.txt"
}
```

**Returns:** Success message

---

#### Directory Operations

##### `create_directory`

Create a directory (including parent directories).

**Arguments:**
```json
{
    "path": "dir/subdir"
}
```

**Returns:** Success message

---

##### `list_directory`

List contents of a directory (non-recursive).

**Arguments:**
```json
{
    "path": "directory/path"
}
```

**Returns:** List of files and directories

---

##### `list_directory_recursive`

List contents of a directory recursively.

**Arguments:**
```json
{
    "path": "directory/path"
}
```

**Returns:** Tree structure of files and directories

---

#### Code Execution

##### `execute_python`

Execute Python code.

**Arguments:**
```json
{
    "code": "print('Hello, World!')"
}
```

**Returns:** Standard output and error from execution

---

##### `execute_bash`

Execute bash commands.

**Arguments:**
```json
{
    "command": "ls -la"
}
```

**Returns:** Command output

---

##### `execute_node`

Execute Node.js code.

**Arguments:**
```json
{
    "code": "console.log('Hello');"
}
```

**Returns:** Standard output from execution

---

##### `execute_ruby`

Execute Ruby code.

**Arguments:**
```json
{
    "code": "puts 'Hello'"
}
```

**Returns:** Standard output from execution

---

#### Plan Management

##### `create_plan`

Create a development plan in `plan.md`.

**Arguments:**
```json
{
    "plan": "## Phase 1\n- [ ] Step 1\n- [ ] Step 2"
}
```

**Returns:** Success message

---

##### `update_plan_step`

Update the status of a plan step.

**Arguments:**
```json
{
    "step_number": 1,
    "new_status": "completed"
}
```

**Returns:** Success message

---

##### `clear_plan`

Clear the current plan.

**Arguments:** None

**Returns:** Success message

---

#### Git Operations

##### `git_status`

Get git repository status.

**Arguments:** None

**Returns:** Git status output

---

## App Module

The app module manages application state.

### Types

#### `App`

Main application state.

```rust
pub struct App {
    pub user_input: String,
    pub conversation: Vec<String>,
    pub status_message: String,
    pub tool_logs: Vec<String>,
    pub is_executing_tool: bool,
    pub current_tool: String,
    pub session_start_time: std::time::Instant,
    pub tokens_used: u64,
    pub total_requests: u64,
    pub total_tools_executed: u64,
    pub conversation_scroll_position: usize,
    pub tool_logs_scroll_position: usize,
    pub is_streaming: bool,
    pub current_streaming_message: String,
}
```

### Methods

#### `new`

```rust
pub fn new() -> Self
```

Creates a new App instance with default values.

---

#### `add_tool_log`

```rust
pub fn add_tool_log(&mut self, log: String)
```

Adds a log entry to the tool logs.

---

#### Usage Tracking

```rust
pub fn increment_tokens(&mut self, tokens: u64)
pub fn increment_requests(&mut self)
pub fn increment_tools_executed(&mut self)
pub fn get_session_duration(&self) -> std::time::Duration
pub fn get_usage_summary(&self) -> String
```

Methods for tracking and reporting usage statistics.

---

#### Scrolling

```rust
pub fn scroll_conversation_up(&mut self)
pub fn scroll_conversation_down(&mut self)
pub fn scroll_conversation_to_top(&mut self)
pub fn scroll_conversation_to_bottom(&mut self)
```

Methods for managing conversation scroll position.

---

#### Streaming

```rust
pub fn start_streaming(&mut self)
pub fn update_streaming_message(&mut self, new_content: &str)
pub fn finish_streaming(&mut self, final_message: String)
```

Methods for managing streaming response state.

---

## Config Module

The config module handles configuration loading.

### Types

#### `Config`

Main configuration structure.

```rust
pub struct Config {
    pub llm: LlmConfig,
}
```

#### `LlmConfig`

LLM-specific configuration.

```rust
pub struct LlmConfig {
    pub provider: Option<String>,
    pub api_key: String,
    pub api_base_url: String,
    pub model_name: String,
}
```

### Methods

#### `from_file`

```rust
pub fn from_file(path: &str) -> Result<Self, io::Error>
```

Loads configuration from a TOML file.

**Parameters:**
- `path`: Path to config file

**Returns:**
- `Ok(Config)`: Loaded configuration
- `Err(io::Error)`: File not found or parse error

**Example:**
```rust
let config = Config::from_file("config.toml")?;
println!("Model: {}", config.llm.model_name);
```

---

## UI Module

The UI module handles terminal rendering.

### Functions

#### `ui`

```rust
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App)
```

Renders the terminal UI for the given app state.

**Parameters:**
- `f`: Ratatui frame for rendering
- `app`: Current application state

**Layout:**
- Top 70%: Conversation area
- Next 20%: Tool logs area
- Next line: Status bar
- Bottom: Input area

**Scrolling:**
- Automatically clamps scroll positions to valid ranges
- Shows indicators when there's more content to scroll

---

## Error Handling

All functions that can fail return `Result` types:

- `Ok(value)`: Successful execution
- `Err(error)`: Error occurred

Error types:
- `Box<dyn std::error::Error>`: Generic error for API/network issues
- `String`: Simple error messages for tool execution
- `io::Error`: File system errors

## Thread Safety

- `Config` and `LlmConfig`: `Clone` + `Send` + `Sync`
- `Message`: `Clone` + `Send` + `Sync`
- `App`: Not thread-safe (single-threaded UI)

## Async Support

Async functions require a Tokio runtime:

```rust
#[tokio::main]
async fn main() {
    let result = send_message(&config, &messages).await;
}
```
