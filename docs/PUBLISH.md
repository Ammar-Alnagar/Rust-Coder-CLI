# Publishing to crates.io

This document explains how to publish `rust_tui_coder` to crates.io.

## Pre-Publication Checklist

 **All tests pass**
```bash
cargo test
# Result: 94 tests passed across 9 test suites
```

 **No clippy warnings**
```bash
cargo clippy --all-targets --all-features
# Result: Clean build with no warnings
```

 **Release build succeeds**
```bash
cargo build --release
# Result: Successful compilation
```

 **Package metadata is complete**
- Name: `rust_tui_coder`
- Version: `1.0.0`
- Description: Comprehensive and descriptive
- License: MIT OR Apache-2.0
- Keywords: Relevant and within limit (5)
- Categories: Appropriate
- README: Present
- Authors: Specified
- Rust version: 1.70 minimum

 **Binary configuration**
```toml
[[bin]]
name = "rust_tui_coder"
path = "src/main.rs"
```

## Installation After Publishing

Users will be able to install the binary using:

```bash
cargo install rust_tui_coder
```

This will install the `rust_tui_coder` binary to `~/.cargo/bin/` (which should be in PATH).

## Usage After Installation

After installation, users can run:

```bash
rust_tui_coder
```

The application will look for a `config.toml` file in the current directory.

## Publishing Steps

1. **Login to crates.io** (one time only):
   ```bash
   cargo login <your-api-token>
   ```
   Get your API token from: https://crates.io/settings/tokens

2. **Verify package contents**:
   ```bash
   cargo package --list
   ```
   This shows what files will be included.

3. **Dry run the publish**:
   ```bash
   cargo publish --dry-run
   ```
   This verifies everything without actually publishing.

4. **Publish to crates.io**:
   ```bash
   cargo publish
   ```

## Post-Publication

After publishing, the package will be available at:
- Main page: `https://crates.io/crates/rust_tui_coder`
- Documentation: `https://docs.rs/rust_tui_coder`
- Repository: Update the repository URL in Cargo.toml with actual GitHub URL

## Version Updates

For future releases:

1. Update version in `Cargo.toml`
2. Run tests: `cargo test`
3. Update CHANGELOG (if you create one)
4. Commit changes
5. Create git tag: `git tag v1.0.1`
6. Publish: `cargo publish`
7. Push tag: `git push origin v1.0.1`

## Excluded Files

The following are excluded from the published package (see `Cargo.toml`):
- `config.toml` (user-specific configuration)
- `plan.md` (runtime generated file)
- `.git` and `.gitignore` (version control)
- `tmp_rovodev_*` (test artifacts)
- `*.png` (images, already in README)

## Important Notes

1. **Configuration Required**: Users need to create a `config.toml` file with their LLM API credentials
2. **First Run**: Include instructions in README for first-time setup
3. **Binary Name**: The binary is named `rust_tui_coder` and can be renamed after installation if desired
4. **Dependencies**: All dependencies are properly specified in Cargo.toml
5. **Platform Support**: Works on Linux, macOS, and Windows (tested on Linux)

## Testing the Installation

After publishing, test the installation in a clean environment:

```bash
# In a new directory
cargo install rust_tui_coder

# Create config file
cat > config.toml << EOF
[llm]
api_key = "your-api-key"
api_base_url = "https://api.openai.com/v1"
model_name = "gpt-4"
EOF

# Run the application
rust_tui_coder
```

## Support and Issues

Direct users to:
- GitHub repository for issues
- README.md for documentation
- config_example.toml for configuration template

## Yanking a Version (if needed)

If you need to yank a published version due to critical bugs:

```bash
cargo yank --vers 1.0.0
```

To un-yank:

```bash
cargo yank --vers 1.0.0 --undo
```

## Current Status

 **Ready for Publication**

All prerequisites have been met:
- Comprehensive test suite (94 tests)
- No clippy warnings
- Clean build
- Proper package metadata
- Documentation included
- Binary configuration correct
