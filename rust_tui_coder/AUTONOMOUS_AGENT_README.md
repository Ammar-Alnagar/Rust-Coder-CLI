# Autonomous Agent CLI

This Rust TUI application features an autonomous AI agent that can complete complex tasks without requiring turn-by-turn interaction.

## Key Features

### 🤖 Autonomous Operation
- **Single Instruction**: Give the agent one instruction and it will work until the task is complete
- **Multi-step Execution**: The agent automatically breaks down complex tasks into steps
- **Tool Usage**: Uses various tools (file operations, commands, code execution) to complete tasks
- **Verification**: Automatically verifies results by reading files or listing directories

### ⏱️ Timeout Protection
- **LLM Timeout**: 30 seconds per LLM request
- **Tool Timeout**: 60 seconds per tool execution
- **Total Timeout**: 10 minutes maximum for entire task
- **Early Stopping**: Stops if task appears complete or if too many iterations

### 🛠️ Available Tools
1. **READ_FILE** - Read file contents
2. **WRITE_FILE** - Create or modify files
3. **RUN_COMMAND** - Execute shell commands
4. **LIST_FILES** - List directory contents
5. **CREATE_DIRECTORY** - Create directories
6. **DELETE_FILE** - Delete files/directories
7. **EXECUTE_CODE** - Execute Python, Bash, or Rust code

## Usage

1. **Start the application**:
   ```bash
   cargo run
   ```

2. **Type your task** and press Enter. Examples:
   - "Create a Python script that prints hello world"
   - "Edit the config.toml file to change the API key"
   - "List all files in the current directory and create a backup"
   - "Read the README.md file and create a summary"

3. **Watch the agent work**:
   - The agent will automatically execute tools
   - Tool logs show each step being performed
   - Status updates indicate progress
   - Task completes when "TASK_COMPLETE:" is shown

## Example Workflow

When you ask: "Create a file called test.txt with the content 'Hello World'"

The agent will:
1. Use `LIST_FILES` to understand current directory
2. Use `WRITE_FILE` to create test.txt
3. Use `READ_FILE` to verify the file was created correctly
4. Respond with "TASK_COMPLETE: Successfully created test.txt..."

## Timeout Handling

The agent has multiple timeout mechanisms to prevent hanging:

- **Per-request timeout**: 30 seconds for LLM calls
- **Tool execution timeout**: 60 seconds for file/command operations
- **Total task timeout**: 10 minutes maximum
- **Iteration limit**: 15 iterations maximum
- **Early detection**: Stops if response indicates completion or confusion

## Status Messages

- 🤖 **Agent is working autonomously...** - Task in progress
- ✅ **Task completed successfully!** - Task finished with TASK_COMPLETE
- ⚠️ **Task stopped early - check results** - Stopped due to timeout/limits
- ⏰ **Execution timed out** - Total timeout reached
- ❌ **Error occurred during execution** - Error during processing

## Configuration

Make sure you have a `config.toml` file with your LLM configuration:

```toml
[llm]
api_url = "your_api_url"
api_key = "your_api_key"
model = "your_model_name"
```

## Tips for Best Results

1. **Be specific**: "Create a Python script that calculates fibonacci numbers" vs "make a script"
2. **Include verification**: The agent will automatically verify its work
3. **Use file paths**: "Edit src/main.rs" vs "edit the main file"
4. **Be patient**: Complex tasks may take several iterations
5. **Check tool logs**: See exactly what the agent is doing

## Troubleshooting

- **Timeout issues**: Try breaking complex tasks into smaller parts
- **Tool failures**: Check if files/directories exist before operations
- **LLM errors**: Verify your API configuration in config.toml
- **Hanging**: The agent will automatically stop after timeouts

The autonomous agent is designed to work efficiently and complete tasks without requiring constant user interaction! 