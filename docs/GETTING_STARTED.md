# Getting Started Guide

Welcome to Rust TUI Coder! This guide will help you get up and running quickly.

## Quick Start

### Prerequisites

- **Rust** 1.70 or higher
- An API key for one of:
  - OpenAI (GPT-3.5, GPT-4)
  - Anthropic (Claude)
  - Local LLM with OpenAI-compatible API

### Installation

#### Option 1: Install from crates.io (Recommended)

```bash
cargo install rust_tui_coder
```

#### Option 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/rust_tui_coder.git
cd rust_tui_coder

# Build the project
cargo build --release

# The binary will be at target/release/rust_tui_coder
```

## Configuration

### Step 1: Create Configuration File

Create a `config.toml` file in your project directory:

```toml
[llm]
api_key = "your-api-key-here"
api_base_url = "https://api.openai.com/v1"
model_name = "gpt-4"
```

You can also use the example configuration:

```bash
cp config_example.toml config.toml
# Edit config.toml with your API key
```

### Step 2: Configure Your Provider

#### For OpenAI

```toml
[llm]
provider = "openai"  # Optional, auto-detected
api_key = "sk-..."
api_base_url = "https://api.openai.com/v1"
model_name = "gpt-4"  # or "gpt-3.5-turbo"
```

#### For Anthropic Claude

```toml
[llm]
provider = "anthropic"
api_key = "sk-ant-..."
api_base_url = "https://api.anthropic.com"
model_name = "claude-3-opus-20240229"
```

#### For Local Models (Ollama, LM Studio, etc.)

```toml
[llm]
provider = "local"
api_key = "not-needed"
api_base_url = "http://localhost:11434/v1"  # Ollama default
model_name = "codellama"
```

### Environment Variables (Alternative)

Instead of `config.toml`, you can use environment variables:

```bash
export LLM_API_KEY="your-api-key"
export LLM_API_BASE_URL="https://api.openai.com/v1"
export LLM_MODEL_NAME="gpt-4"
```

## First Run

### Launch the Application

```bash
rust_tui_coder
```

Or if built from source:

```bash
./target/release/rust_tui_coder
```

### Initial Screen

You'll see a terminal interface with:
- **Conversation area** (top) - Shows your chat with the AI
- **Tool logs area** (middle) - Shows tool execution details
- **Status bar** - Shows available commands
- **Input area** (bottom) - Where you type your messages

### Your First Interaction

1. Type a message in the input area:
   ```
   Create a hello world program in Python
   ```

2. Press **Enter** to send

3. Watch as the AI:
   - Generates code
   - Uses tools (like `write_file`)
   - Executes the code
   - Shows you the results

## Basic Usage

### Sending Messages

1. Type your message at the bottom
2. Press **Enter** to send
3. Watch the response in the conversation area

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Enter` | Send message |
| `↑` / `↓` | Scroll conversation |
| `PgUp` / `PgDn` | Page up/down |
| `Home` | Scroll to top |
| `End` | Scroll to bottom |
| `Ctrl+C` | Quit application |

### Special Commands

| Command | Description |
|---------|-------------|
| `/quit` | Exit the application |
| `/stats` | Show session statistics |

## Common Tasks

### Example 1: Create a File

```
Create a file named hello.py with a hello world program
```

The AI will:
1. Write the code
2. Save it to `hello.py`
3. Confirm the file was created

### Example 2: Read and Modify a File

```
Read example.txt and add a timestamp at the beginning
```

The AI will:
1. Read the file
2. Add a timestamp
3. Update the file
4. Show you the changes

### Example 3: Execute Code

```
Write a Python script to calculate fibonacci numbers and run it
```

The AI will:
1. Write the script
2. Save it to a file
3. Execute it
4. Show you the output

### Example 4: Git Operations

```
Show me the current git status
```

The AI will use the `git_status` tool to show repository status.

### Example 5: Create a Development Plan

```
Create a plan to build a REST API with user authentication
```

The AI will:
1. Create a structured plan
2. Save it to `plan.md`
3. You can ask it to implement steps one by one

## Understanding Tool Execution

When the AI needs to perform actions, it uses **tools**:

### File Tools
- `read_file` - Read file contents
- `write_file` - Create/overwrite file
- `append_to_file` - Add to end of file
- `search_and_replace` - Find and replace text
- `delete_file` - Remove file

### Directory Tools
- `create_directory` - Create folders
- `list_directory` - List folder contents
- `list_directory_recursive` - Show folder tree

### Execution Tools
- `execute_python` - Run Python code
- `execute_bash` - Run shell commands
- `execute_node` - Run JavaScript
- `execute_ruby` - Run Ruby code

### Planning Tools
- `create_plan` - Make implementation plan
- `update_plan_step` - Mark steps complete
- `clear_plan` - Remove current plan

### Version Control
- `git_status` - Check git status

Tool execution is shown in the **Tool Logs** area.

## Tips for Effective Use

### 1. Be Specific

❌ "Make a website"
✅ "Create an HTML file with a form that collects name and email"

### 2. Break Down Complex Tasks

Instead of asking for everything at once, work step-by-step:
1. "Create the project structure"
2. "Implement the database models"
3. "Add the API endpoints"

### 3. Use the Plan Feature

For complex projects:
```
Create a plan to build a todo list application with React and Express
```

Then:
```
Implement step 1 of the plan
```

### 4. Review Tool Logs

The tool logs area shows exactly what the AI is doing. Check it to:
- Verify file operations
- See command outputs
- Understand execution results

### 5. Iterative Development

You can refine the AI's work:
```
The function is good but add error handling
```

### 6. Ask for Explanations

```
Explain what this code does
```

or

```
Why did you use this approach?
```

## Session Management

### View Statistics

Type `/stats` to see:
- Session duration
- Tokens used
- Number of requests
- Tools executed
- Average tokens per request

### Scroll Through History

- Use **↑/↓** to scroll through conversation
- Use **PgUp/PgDn** for faster scrolling
- Use **Home/End** to jump to top/bottom

### Clear the Plan

If you want to start a new plan:
```
Clear the current plan
```

## Troubleshooting

### "Config file not found"

**Solution:** Create a `config.toml` file in the directory where you run the app.

```bash
cp config_example.toml config.toml
# Edit with your API key
```

### "API key invalid"

**Solution:** Check your API key in `config.toml`:
- OpenAI keys start with `sk-`
- Anthropic keys start with `sk-ant-`
- Ensure no extra spaces or quotes

### "Connection refused"

**Solution:** Check your `api_base_url`:
- OpenAI: `https://api.openai.com/v1`
- Anthropic: `https://api.anthropic.com`
- Local: Ensure your local server is running

### "Tool execution failed"

**Solution:** Check the tool logs for details. Common issues:
- File permissions
- Missing dependencies (Python, Node, etc.)
- Invalid file paths

### Terminal Display Issues

**Solution:** 
- Ensure your terminal supports UTF-8
- Try a different terminal emulator
- Check terminal size (minimum 80x24 recommended)

### Garbled Text

**Solution:**
- Your terminal may not support all features
- Try running: `export TERM=xterm-256color`

## Advanced Configuration

### Custom Model Parameters

While not directly supported in config, you can mention preferences:
```
Please be concise in your responses
```

### Working Directory

The app operates in the directory where you launch it. To work on a specific project:

```bash
cd /path/to/your/project
rust_tui_coder
```

### Multiple Projects

Create a `config.toml` in each project directory, or use environment variables:

```bash
cd project1
LLM_MODEL_NAME="gpt-3.5-turbo" rust_tui_coder
```

## Next Steps

Now that you're set up, explore these resources:

- **[README.md](README.md)** - Full feature documentation
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - System design details
- **[API.md](API.md)** - API reference
- **[EXAMPLES.md](EXAMPLES.md)** - More usage examples

## Getting Help

### Check Documentation
- Read the README for detailed features
- Check ARCHITECTURE.md for system internals
- Review API.md for technical details

### Common Questions

**Q: How much does it cost?**
A: Cost depends on your LLM provider and usage. Check with OpenAI/Anthropic for pricing.

**Q: Can I use it offline?**
A: Yes, with a local model (Ollama, LM Studio).

**Q: Is my code safe?**
A: Code is processed by your chosen LLM provider. Read their privacy policies.

**Q: Can I customize the tools?**
A: Currently, tools are built-in. Custom tools require modifying the source code.

**Q: What languages can I execute?**
A: Python, Bash, Node.js, and Ruby are supported out of the box.

## Support

- **Issues**: Report bugs on GitHub
- **Features**: Suggest features via GitHub issues
- **Documentation**: All docs are in the `docs/` folder

Happy coding! 🦀✨
