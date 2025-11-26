# New Features in v0.3.0

This document highlights the new features added in version 0.3.0 of Rust TUI Coder.

##  ReAct Pattern: Reasoning Before Acting

The AI now follows the **ReAct (Reasoning + Acting)** pattern, which means it explicitly thinks through problems before taking action.

### What This Means

- **Better Decision Making**: The AI reasons about what needs to be done
- **Transparent Process**: You can see the AI's thought process
- **Error Recovery**: Can adapt when things don't go as planned
- **Systematic Approach**: Complex tasks are broken down logically

### Example

```
You: Create a REST API with database support

AI REASONING: This is a complex task requiring multiple steps. Let me break it down:
1. Choose appropriate framework and database
2. Set up project structure
3. Configure database connection
4. Create API endpoints
5. Add error handling
6. Test the API

ACTION: TOOL: {"name": "CREATE_PLAN", "parameters": {...}}
```

**Learn more**: [ReAct Pattern Documentation](REACT_PATTERN.md)

---

##  OS Detection & Cross-Platform Adaptation

The assistant now automatically detects your operating system and adapts its behavior accordingly.

### GET_OS_INFO Tool

Get detailed information about your system:

```
You: What system am I running on?

AI: [Uses GET_OS_INFO]
Operating System Information:
• OS: Linux (Ubuntu 22.04)
• Architecture: x86_64
• OS Family: unix
• Shell: bash / sh
• Path Separator: /
• Command Separator: &&
• Temp Directory: /tmp
```

### Automatic Command Adaptation

The AI now automatically uses the right commands for your OS:

**On Linux/macOS:**
```bash
ls -la                 # List files
cp file.txt backup/    # Copy files
rm -rf directory/      # Delete directory
```

**On Windows:**
```cmd
dir /s                 # List files
copy file.txt backup\  # Copy files
rmdir /s directory\    # Delete directory
```

### Cross-Platform File Operations

- Automatic path separator handling (`/` vs `\`)
- OS-appropriate shell commands
- Platform-specific tool availability checks

---

##  Time & Date Awareness

The AI can now access the current date and time from your system.

### GET_TIME Tool

```
You: What's the current date and time?

AI: [Uses GET_TIME]
Current Date & Time:
• Date: 2024-01-15
• Time: 14:35:22
• Timezone: EST
• Unix Timestamp: 1705340122
```

### Use Cases

- **Timestamped Logs**: Create logs with accurate timestamps
- **Scheduling**: Make time-aware decisions
- **Date Calculations**: Work with dates and times in code
- **File Naming**: Generate files with date/time in names

### Example

```
You: Create a backup of config.json with today's date in the filename

AI REASONING: I'll get the current date and use it in the filename.
ACTION: [Uses GET_TIME to get date]
OBSERVATION: Date is 2024-01-15
ACTION: [Uses COPY_FILE]
Successfully copied 'config.json' to 'config.json.2024-01-15.backup'
```

---

##  Enhanced File Operations

Three new tools for better file management:

### COPY_FILE

Copy files easily:

```json
TOOL: {"name": "COPY_FILE", "parameters": {
  "source": "important.txt",
  "destination": "backup/important.txt"
}}
```

### MOVE_FILE

Move or relocate files:

```json
TOOL: {"name": "MOVE_FILE", "parameters": {
  "source": "old/location/file.txt",
  "destination": "new/location/file.txt"
}}
```

### RENAME_FILE

Rename files or directories:

```json
TOOL: {"name": "RENAME_FILE", "parameters": {
  "old_name": "draft.txt",
  "new_name": "final.txt"
}}
```

### Examples

```
You: Make a backup copy of my database file

AI: [Uses COPY_FILE]
Successfully copied 'database.db' to 'database.db.backup'

---

You: Rename my project folder to match the new name

AI: [Uses RENAME_FILE]
Successfully renamed 'old-project-name' to 'new-project-name'
```

---

##  Custom System Prompts (prompt.md)

You can now customize the AI's behavior by creating a `prompt.md` file in your working directory.

### How It Works

1. Create a `prompt.md` file:
   ```bash
   cp prompt.md.example prompt.md
   ```

2. Add your custom instructions:
   ```markdown
   # My Coding Preferences
   
   ## Style Guide
   - Always use TypeScript, never JavaScript
   - Prefer functional programming
   - Use async/await over promises
   
   ## Project Rules
   - All functions must have JSDoc comments
   - Write tests for every feature
   - Use Prettier for formatting
   ```

3. Start the application - it automatically loads your instructions!

### What You Can Customize

- **Code Style**: Naming conventions, formatting preferences
- **Technology Choices**: Preferred frameworks, libraries, tools
- **Project Conventions**: Commit message format, file structure
- **Communication Style**: How the AI explains things
- **Security Rules**: Specific security requirements
- **Documentation Standards**: How code should be documented

### Example

With this in your `prompt.md`:
```markdown
## Technology Preferences
- Use React with TypeScript for frontend
- Use PostgreSQL for database
```

When you ask:
```
You: Create a web application

AI REASONING: Based on the custom instructions, I'll use React with
TypeScript for the frontend and PostgreSQL for the database.
```

**See**: [prompt.md.example](../prompt.md.example) for a complete template

---

##  Improved Documentation

New comprehensive documentation added:

### README_FULL.md

A complete, all-in-one documentation file covering:
- Installation and setup
- All 27 tools
- Configuration options
- Usage examples
- Troubleshooting
- Contributing guidelines

### REACT_PATTERN.md

Detailed explanation of the ReAct pattern:
- How it works
- Benefits
- Examples
- Best practices
- Implementation details

### CHANGELOG.md

Version history and migration guides:
- What's new in each version
- Breaking changes
- Migration instructions
- Future plans

---

##  Technical Improvements

### Under the Hood

- **OS-Adaptive Command Execution**: `RUN_COMMAND` now uses appropriate shell
- **Chrono Integration**: Professional date/time handling
- **Enhanced Prompts**: Better structured with escape sequences
- **Improved Error Messages**: More helpful feedback
- **Better Logging**: New tools included in execution logs

### System Requirements

No new requirements! Works on:
-  Linux (all distributions)
-  macOS (Intel and Apple Silicon)
-  Windows 10/11

---

##  Migration from v0.2.x

Upgrading is seamless - no breaking changes!

```bash
# Update via cargo
cargo install rust_tui_coder --force

# (Optional) Set up custom prompt
cp prompt.md.example prompt.md
nano prompt.md  # Edit to your preferences

# Start using new features immediately!
rust_tui_coder
```

---

##  Quick Start with New Features

### Try ReAct Pattern

```
You: Help me understand how to use the ReAct pattern

AI REASONING: I should explain ReAct with a practical example...
ACTION: [Provides explanation]
```

### Check Your System

```
You: Show me my system information

AI: [Uses GET_OS_INFO]
[Displays detailed OS information]
```

### Get Current Time

```
You: What time is it?

AI: [Uses GET_TIME]
[Shows current date, time, and timezone]
```

### Copy Important Files

```
You: Create a backup of all my config files

AI REASONING: I'll find config files and create backups...
ACTION: [Uses LIST_FILES to find configs]
ACTION: [Uses COPY_FILE for each one]
```

### Use Custom Prompts

```bash
# Create your prompt file
echo "Always explain your code in detail" > prompt.md

# The AI will now follow your instruction!
rust_tui_coder
```

---

##  Learn More

- **[Complete README](README_FULL.md)** - Everything in one place
- **[ReAct Pattern Guide](REACT_PATTERN.md)** - Deep dive into reasoning
- **[Examples](EXAMPLES.md)** - 30+ practical examples
- **[Architecture](ARCHITECTURE.md)** - How it all works
- **[Changelog](../CHANGELOG.md)** - Version history

---

##  What's Next?

See [CHANGELOG.md](../CHANGELOG.md) for planned features in future releases!

**Enjoy the enhanced Rust TUI Coder! **
