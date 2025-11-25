# Troubleshooting Guide

This guide helps you diagnose and fix common issues with Rust TUI Coder.

## Table of Contents

1. [Installation Issues](#installation-issues)
2. [Configuration Problems](#configuration-problems)
3. [API Connection Issues](#api-connection-issues)
4. [Tool Execution Errors](#tool-execution-errors)
5. [UI/Display Problems](#uidisplay-problems)
6. [Performance Issues](#performance-issues)
7. [Platform-Specific Issues](#platform-specific-issues)

---

## Installation Issues

### Error: "Package not found"

**Problem:** `cargo install rust_tui_coder` fails

**Solutions:**

1. **Update Rust**
   ```bash
   rustup update stable
   ```

2. **Check Rust version**
   ```bash
   rustc --version  # Should be 1.70 or higher
   ```

3. **Clear cargo cache**
   ```bash
   rm -rf ~/.cargo/registry/index/*
   cargo install rust_tui_coder
   ```

---

### Error: "Failed to compile"

**Problem:** Build fails during installation

**Solutions:**

1. **Check system dependencies**
   ```bash
   # Ubuntu/Debian
   sudo apt-get install build-essential pkg-config libssl-dev
   
   # macOS
   xcode-select --install
   
   # Fedora
   sudo dnf install gcc openssl-devel
   ```

2. **Use specific version**
   ```bash
   cargo install rust_tui_coder --version 1.0.0
   ```

3. **Build from source**
   ```bash
   git clone https://github.com/yourusername/rust_tui_coder.git
   cd rust_tui_coder
   cargo build --release
   ./target/release/rust_tui_coder
   ```

---

## Configuration Problems

### Error: "Config file not found"

**Problem:** Application can't find `config.toml`

**Diagnostic:**
```bash
# Check current directory
pwd

# List files
ls -la | grep config
```

**Solutions:**

1. **Create config file**
   ```bash
   cat > config.toml << EOF
   [llm]
   api_key = "your-api-key"
   api_base_url = "https://api.openai.com/v1"
   model_name = "gpt-4"
   EOF
   ```

2. **Use example config**
   ```bash
   cp config_example.toml config.toml
   # Edit with your details
   nano config.toml
   ```

3. **Check file location**
   - Config must be in the directory where you run the app
   - Or use environment variables instead

---

### Error: "Invalid TOML format"

**Problem:** Config file has syntax errors

**Diagnostic:**
```bash
# Validate TOML
cat config.toml
```

**Common Issues:**

1. **Missing quotes**
   ```toml
   # Wrong
   api_key = sk-abc123
   
   # Correct
   api_key = "sk-abc123"
   ```

2. **Wrong section headers**
   ```toml
   # Wrong
   [LLM]
   
   # Correct
   [llm]
   ```

3. **Extra characters**
   ```toml
   # Wrong
   api_key = "key",
   
   # Correct
   api_key = "key"
   ```

**Solution:**
```toml
# Minimal valid config
[llm]
api_key = "your-key-here"
api_base_url = "https://api.openai.com/v1"
model_name = "gpt-4"
```

---

### Error: "Missing required field"

**Problem:** Config is missing a required field

**Solution:** Ensure all required fields are present:

```toml
[llm]
api_key = "required"
api_base_url = "required"
model_name = "required"
provider = "optional"
```

---

## API Connection Issues

### Error: "Invalid API key"

**Problem:** API authentication fails

**Diagnostic:**
```bash
# Check API key format
cat config.toml | grep api_key
```

**Solutions:**

1. **OpenAI keys**
   - Should start with `sk-`
   - Get from: https://platform.openai.com/api-keys
   - Check for extra spaces or newlines

2. **Anthropic keys**
   - Should start with `sk-ant-`
   - Get from: https://console.anthropic.com/

3. **Verify key is active**
   ```bash
   # Test OpenAI key
   curl https://api.openai.com/v1/models \
     -H "Authorization: Bearer YOUR_API_KEY"
   ```

---

### Error: "Connection refused" or "Connection timeout"

**Problem:** Can't connect to API server

**Diagnostic:**
```bash
# Test connectivity
ping api.openai.com

# Test API endpoint
curl -I https://api.openai.com/v1/models
```

**Solutions:**

1. **Check internet connection**
   ```bash
   ping google.com
   ```

2. **Check firewall**
   - Ensure port 443 (HTTPS) is open
   - Check corporate firewall/proxy

3. **Verify API base URL**
   ```toml
   # OpenAI
   api_base_url = "https://api.openai.com/v1"
   
   # Anthropic
   api_base_url = "https://api.anthropic.com"
   
   # Local (Ollama)
   api_base_url = "http://localhost:11434/v1"
   ```

4. **Use proxy (if needed)**
   ```bash
   export HTTPS_PROXY=http://proxy.company.com:8080
   rust_tui_coder
   ```

---

### Error: "Rate limit exceeded"

**Problem:** Too many API requests

**Solutions:**

1. **Wait and retry**
   - OpenAI: Wait 60 seconds
   - Anthropic: Check your rate limits

2. **Check your usage**
   - OpenAI: https://platform.openai.com/usage
   - Anthropic: https://console.anthropic.com/

3. **Use slower model**
   ```toml
   model_name = "gpt-3.5-turbo"  # Instead of gpt-4
   ```

---

### Error: "Model not found"

**Problem:** Specified model doesn't exist or you don't have access

**Solutions:**

1. **Check model name spelling**
   ```toml
   # Correct
   model_name = "gpt-4"
   
   # Wrong
   model_name = "gpt4"
   model_name = "gpt-4-turbo"  # Use correct model ID
   ```

2. **Verify model access**
   - GPT-4: Requires paid OpenAI account
   - Claude: Check Anthropic access

3. **Use available model**
   ```toml
   # OpenAI (always available)
   model_name = "gpt-3.5-turbo"
   
   # Anthropic
   model_name = "claude-3-opus-20240229"
   ```

---

## Tool Execution Errors

### Error: "Permission denied" (File operations)

**Problem:** Can't read/write files

**Diagnostic:**
```bash
# Check file permissions
ls -la filename

# Check directory permissions
ls -la .
```

**Solutions:**

1. **Check ownership**
   ```bash
   # Make file writable
   chmod 644 filename
   
   # Make directory writable
   chmod 755 directory
   ```

2. **Run with correct user**
   ```bash
   # Don't run as root unnecessarily
   whoami
   ```

3. **Check file locks**
   - Close other programs using the file
   - Check for `.lock` files

---

### Error: "Command not found" (Code execution)

**Problem:** Required interpreter not installed

**Diagnostic:**
```bash
# Check if installed
python3 --version
node --version
ruby --version
```

**Solutions:**

1. **Install missing interpreter**
   ```bash
   # Python
   sudo apt-get install python3  # Ubuntu
   brew install python3          # macOS
   
   # Node.js
   sudo apt-get install nodejs
   brew install node
   
   # Ruby
   sudo apt-get install ruby
   brew install ruby
   ```

2. **Check PATH**
   ```bash
   echo $PATH
   which python3
   ```

---

### Error: "No such file or directory"

**Problem:** Tool can't find specified file

**Diagnostic:**
```bash
# Check if file exists
ls -la path/to/file

# Check current directory
pwd
```

**Solutions:**

1. **Use correct path**
   - Relative paths: `src/main.rs`
   - Absolute paths: `/home/user/project/src/main.rs`

2. **Create missing directories**
   ```bash
   mkdir -p path/to/directory
   ```

3. **Check working directory**
   - App runs in the directory where you launched it
   - Use `/stats` to see current context

---

## UI/Display Problems

### Problem: Garbled text or escape sequences

**Symptoms:**
- Strange characters: `^[[0m`
- Colors don't work
- Box drawing characters broken

**Solutions:**

1. **Set TERM variable**
   ```bash
   export TERM=xterm-256color
   rust_tui_coder
   ```

2. **Use compatible terminal**
   - ✅ Good: iTerm2, GNOME Terminal, Alacritty, Windows Terminal
   - ❌ Poor: Basic terminals, old terminal emulators

3. **Check terminal capabilities**
   ```bash
   echo $TERM
   tput colors  # Should show 256
   ```

---

### Problem: UI doesn't update / frozen screen

**Solutions:**

1. **Restart application**
   - Press `Ctrl+C` to quit
   - Launch again

2. **Check terminal size**
   ```bash
   # Terminal should be at least 80x24
   tput cols  # Should be >= 80
   tput lines # Should be >= 24
   ```

3. **Resize terminal window**
   - Make it larger
   - Restart app after resizing

---

### Problem: Can't see cursor or input

**Solutions:**

1. **Check input area**
   - Scroll to bottom with `End` key
   - Input area is at the very bottom

2. **Clear screen**
   - Restart the application
   - Check if other programs are interfering

---

### Problem: Scrolling doesn't work

**Solutions:**

1. **Use correct keys**
   - `↑` / `↓`: Line by line
   - `PgUp` / `PgDn`: Page by page
   - `Home` / `End`: Top / Bottom

2. **Check if conversation has content**
   - No scrolling if conversation is short

3. **Focus on correct area**
   - Scrolling affects conversation area only

---

## Performance Issues

### Problem: Slow response times

**Diagnostic:**
- Check usage with `/stats`
- Note: First request is always slower (cold start)

**Solutions:**

1. **Check model**
   ```toml
   # Faster
   model_name = "gpt-3.5-turbo"
   
   # Slower but better
   model_name = "gpt-4"
   ```

2. **Check network**
   ```bash
   # Test latency
   ping api.openai.com
   
   # Test speed
   curl -w "@curl-format.txt" -o /dev/null -s https://api.openai.com/v1/models
   ```

3. **Use local model**
   ```bash
   # Install Ollama
   curl -fsSL https://ollama.com/install.sh | sh
   
   # Run a model
   ollama run codellama
   
   # Configure
   [llm]
   api_base_url = "http://localhost:11434/v1"
   model_name = "codellama"
   ```

---

### Problem: High memory usage

**Diagnostic:**
```bash
# Check memory
top | grep rust_tui_coder
```

**Solutions:**

1. **Restart application periodically**
   - Long conversations use more memory
   - Quit and restart to clear

2. **Limit conversation length**
   - Keep interactions focused
   - Start new sessions for new tasks

---

### Problem: Application crashes

**Diagnostic:**
- Check error message
- Look for panic messages

**Solutions:**

1. **Update to latest version**
   ```bash
   cargo install rust_tui_coder --force
   ```

2. **Check logs**
   ```bash
   # Run with backtrace
   RUST_BACKTRACE=1 rust_tui_coder
   ```

3. **Report bug**
   - Create GitHub issue with:
     - Error message
     - Steps to reproduce
     - System information

---

## Platform-Specific Issues

### Linux Issues

**Problem: "Cannot open shared object file"**

**Solution:**
```bash
# Install missing libraries
sudo apt-get install libssl-dev pkg-config
```

---

**Problem: Terminal colors don't work**

**Solution:**
```bash
# Install 256-color support
sudo apt-get install ncurses-term
export TERM=xterm-256color
```

---

### macOS Issues

**Problem: "xcrun: error: invalid active developer path"**

**Solution:**
```bash
xcode-select --install
```

---

**Problem: OpenSSL linking errors**

**Solution:**
```bash
brew install openssl
export OPENSSL_DIR=$(brew --prefix openssl)
cargo build --release
```

---

### Windows Issues

**Problem: PowerShell execution policy**

**Solution:**
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

---

**Problem: ANSI colors not working**

**Solution:**
1. Use Windows Terminal (not cmd.exe)
2. Enable ANSI colors:
   ```powershell
   Set-ItemProperty HKCU:\Console VirtualTerminalLevel -Type DWORD 1
   ```

---

**Problem: Git commands don't work**

**Solution:**
- Install Git for Windows
- Ensure git.exe is in PATH

---

## Advanced Diagnostics

### Enable Debug Logging

```bash
# Run with verbose output
RUST_LOG=debug rust_tui_coder

# Or with backtrace
RUST_BACKTRACE=full rust_tui_coder
```

### Test API Connection Manually

```bash
# OpenAI
curl https://api.openai.com/v1/models \
  -H "Authorization: Bearer YOUR_API_KEY"

# Anthropic
curl https://api.anthropic.com/v1/messages \
  -H "x-api-key: YOUR_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "content-type: application/json" \
  -d '{
    "model": "claude-3-opus-20240229",
    "max_tokens": 1024,
    "messages": [{"role": "user", "content": "Hello"}]
  }'
```

### Verify Installation

```bash
# Check installed version
cargo install --list | grep rust_tui_coder

# Verify binary
which rust_tui_coder
rust_tui_coder --version  # If version flag is implemented
```

---

## Getting Help

If you're still experiencing issues:

1. **Check documentation**
   - [README.md](README.md) - General information
   - [GETTING_STARTED.md](GETTING_STARTED.md) - Setup guide
   - [API.md](API.md) - API reference

2. **Search existing issues**
   - Check GitHub issues for similar problems
   - See if solution already exists

3. **Create an issue**
   - Provide detailed information:
     - Operating system and version
     - Rust version (`rustc --version`)
     - Application version
     - Configuration (remove API key!)
     - Complete error message
     - Steps to reproduce
   - Use the bug report template

4. **Community help**
   - Join discussions on GitHub
   - Ask in community forums

---

## Common Pitfalls

### Pitfall 1: Wrong working directory

**Problem:** Files aren't where you expect

**Solution:** Application runs in the directory where you launch it
```bash
cd /path/to/your/project
rust_tui_coder
```

---

### Pitfall 2: API key in config has extra characters

**Problem:** Copy-paste added spaces or newlines

**Solution:** Check carefully
```toml
# Wrong - has space at end
api_key = "sk-abc123 "

# Correct
api_key = "sk-abc123"
```

---

### Pitfall 3: Expecting instant results with GPT-4

**Problem:** GPT-4 can take 10-30 seconds to respond

**Solution:** This is normal! GPT-4 is slower but more capable

---

### Pitfall 4: Not using quotes in TOML

**Problem:** String values without quotes

**Solution:** Always quote strings in TOML
```toml
model_name = "gpt-4"  # Not: model_name = gpt-4
```

---

**Still need help?** Open an issue on GitHub with detailed information!
