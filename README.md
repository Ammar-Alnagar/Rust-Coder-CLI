# Rust TUI Coder

> ** Documentation has been reorganized into the `docs/` folder for better organization.**

**[View Complete README ->](docs/README_FULL.md)**

---

## Quick Links

###  Documentation
- **[Complete README](docs/README_FULL.md)** - Full project documentation
- **[Documentation Index](docs/INDEX.md)** - Navigation guide for all docs
- **[Getting Started Guide](docs/GETTING_STARTED.md)** - Installation and setup
- **[Examples](docs/EXAMPLES.md)** - Practical usage examples
- **[Troubleshooting](docs/TROUBLESHOOTING.md)** - Fix common issues

### 👨‍💻 For Developers
- **[Architecture](docs/ARCHITECTURE.md)** - System design and structure
- **[API Reference](docs/API.md)** - Complete API documentation
- **[Contributing](docs/CONTRIBUTING.md)** - Contribution guidelines
- **[Testing](docs/TESTING.md)** - Test suite documentation

### 📦 For Maintainers
- **[Publishing Guide](docs/PUBLISH.md)** - Publishing to crates.io
- **[Improvements Summary](docs/IMPROVEMENTS_SUMMARY.md)** - Recent changes
- **[Completion Report](docs/COMPLETION_REPORT.md)** - Development milestones

---

##  Quick Start

### Installation

```bash
cargo install rust_tui_coder
```

### Configuration

Create a `config.toml` file:

```toml
[llm]
api_key = "your-api-key-here"
api_base_url = "https://api.openai.com/v1"
model_name = "gpt-4"
```

### Run

```bash
rust_tui_coder
```

---

##  What is Rust TUI Coder?

A powerful terminal-based AI coding assistant that brings AI intelligence directly to your command line. Built with Rust for performance and reliability.

### Key Features

-  **Natural Language Interface** - Chat with AI about your code
-  **Direct Tool Execution** - AI manipulates files and runs commands
-  **Real-time Monitoring** - Watch tool execution in real-time
-  **Beautiful TUI** - Modern terminal interface with ratatui
-  **Multiple LLM Support** - OpenAI, Anthropic, or local models
-  **Fast & Efficient** - Built with Rust for performance

### Quick Example

```
You: Create a Python script that calculates fibonacci numbers and run it

AI: I'll create the script and execute it...
[Creates fibonacci.py]
[Executes the script]
[Shows output: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34...]
```

---

##  Available Tools

- **File Operations**: read, write, append, search/replace, delete
- **Directory Operations**: create, list, recursive list
- **Code Execution**: Python, Bash, Node.js, Ruby
- **Plan Management**: create plans, update steps, clear plans
- **Git Operations**: status checking

---

##  Documentation Structure

The documentation is organized in the `docs/` folder:

```
docs/
├── README_FULL.md           # Complete project README
├── INDEX.md                 # Documentation navigation
├── GETTING_STARTED.md       # Setup and first steps
├── EXAMPLES.md              # 30+ usage examples
├── ARCHITECTURE.md          # System design
├── API.md                   # Complete API reference
├── CONTRIBUTING.md          # How to contribute
├── TESTING.md               # Test suite (94 tests)
├── TROUBLESHOOTING.md       # Common issues and fixes
├── PUBLISH.md               # Publishing guide
├── IMPROVEMENTS_SUMMARY.md  # Recent improvements
└── COMPLETION_REPORT.md     # Development report
```

**Start here:** [Documentation Index](docs/INDEX.md)

---

##  Contributing

We welcome contributions! See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines.

---

##  License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

## 🔗 Resources

- **[Full Documentation](docs/INDEX.md)** - Complete documentation guide
- **[Getting Started](docs/GETTING_STARTED.md)** - New user guide
- **[Examples](docs/EXAMPLES.md)** - Real-world usage examples

---

**Happy Coding! **
