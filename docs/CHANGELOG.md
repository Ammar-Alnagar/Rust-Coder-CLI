# Changelog

All notable changes to Rust TUI Coder will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2024-01-XX

### Added

#### ReAct Pattern Implementation
- **Reasoning + Acting**: AI now explicitly reasons before taking actions
- Improved decision-making through systematic thinking
- Better error recovery and adaptation
- Transparent thought process visible to users

#### OS Detection & Adaptation
- **GET_OS_INFO** tool: Detect operating system, architecture, and shell type
- Automatic command adaptation for Windows, Linux, and macOS
- OS-aware path separators and command syntax
- Shell detection (bash/sh for Unix, cmd.exe/PowerShell for Windows)

#### Time & Date Tools
- **GET_TIME** tool: Access current date, time, timezone, and Unix timestamp
- Time-aware operations for scheduling and timestamps
- Timezone information included in responses

#### Enhanced File Operations
- **COPY_FILE**: Copy files from source to destination
- **MOVE_FILE**: Move or relocate files and directories
- **RENAME_FILE**: Rename files and directories
- Better file manipulation capabilities

#### Custom System Prompts
- **prompt.md** support: Users can add custom instructions
- Personalized AI behavior and preferences
- Project-specific conventions and style guides
- Automatic loading of custom prompts at startup

#### Documentation Improvements
- Added comprehensive **README_FULL.md**
- New **REACT_PATTERN.md** explaining the reasoning pattern
- **prompt.md.example** template for custom instructions
- Updated INDEX.md with new features
- Added CHANGELOG.md for version tracking

### Changed

- **System Prompt**: Completely redesigned with ReAct pattern emphasis
- **RUN_COMMAND**: Now OS-adaptive, automatically uses correct shell
- Tool execution logging now includes new tools
- Improved error messages and user feedback

### Enhanced

- Better cross-platform compatibility
- More transparent AI decision-making process
- Improved tool reliability and error handling
- Enhanced documentation and examples

### Technical Details

- Added `chrono` dependency for time operations
- Improved format string handling in prompts
- Enhanced tool enum with new variants
- Better OS detection using `std::env::consts`

## [0.2.1] - Previous Release

### Features
- Terminal-based user interface with Ratatui
- Real-time streaming responses
- 22 development tools
- Planning system for complex tasks
- Multi-language code execution
- Git integration
- Session statistics tracking

### Tools
- File operations (read, write, append, search/replace, delete)
- Directory operations (list, create, recursive list)
- Search tools (grep, glob)
- Code execution (Python, JavaScript, Rust, Go, Java, C/C++)
- Development workflow (git, linting, testing)
- Package management
- Planning tools

## Future Plans

### Planned for 0.4.0
- [ ] Multi-file editing support
- [ ] Workspace management
- [ ] Enhanced git operations (branch, merge, pull)
- [ ] Code refactoring tools
- [ ] Database query tools
- [ ] API testing tools
- [ ] Docker integration
- [ ] Configuration profiles

### Under Consideration
- [ ] Plugin system
- [ ] Custom tool definitions
- [ ] Conversation history persistence
- [ ] Multiple concurrent LLM sessions
- [ ] Web-based UI option
- [ ] Voice input support
- [ ] IDE integrations

## Migration Guide

### Upgrading from 0.2.x to 0.3.0

No breaking changes! The upgrade is seamless:

1. Update your installation:
   ```bash
   cargo install rust_tui_coder --force
   ```

2. (Optional) Create a `prompt.md` file for custom instructions:
   ```bash
   cp prompt.md.example prompt.md
   # Edit prompt.md with your preferences
   ```

3. Enjoy the new features!

### New Tool Usage Examples

#### Using GET_OS_INFO
```
You: What operating system am I using?
AI: Let me check...
[Uses GET_OS_INFO tool]
```

#### Using GET_TIME
```
You: What's the current date and time?
AI: [Uses GET_TIME tool]
Current Date & Time:
• Date: 2024-01-15
• Time: 14:30:45
• Timezone: EST
```

#### Using Enhanced File Operations
```
You: Make a backup of config.json
AI: [Uses COPY_FILE]
Successfully copied 'config.json' to 'config.json.backup'
```

## Contributors

Thank you to all contributors who made this release possible!

---

For detailed information about any feature, see the [documentation](docs/INDEX.md).
