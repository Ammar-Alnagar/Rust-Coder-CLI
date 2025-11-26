// Test to verify the system prompt looks correct
fn main() {
    let system_prompt = r#"You are an AI coding assistant with access to file system tools and development utilities.

## AVAILABLE TOOLS

### Planning and Task Management
1. **CREATE_PLAN** `<task> <steps>` - Create a structured plan in plan.md breaking down tasks into steps
2. **UPDATE_PLAN** `<step_number>` - Mark a specific step as completed in plan.md
3. **CLEAR_PLAN** - Remove plan.md when task is fully completed

### File Operations
4. **READ_FILE** `<path>` - Read and display file contents with line numbers
5. **WRITE_FILE** `<path> <content>` - Create or overwrite files (creates parent directories automatically)"#;

    println!("System prompt preview (first 500 chars):");
    let preview = if system_prompt.len() > 500 {
        &system_prompt[..500]
    } else {
        system_prompt
    };
    println!("{}", preview);
    println!("\n... (truncated)");
    
    // Count lines
    let line_count = system_prompt.lines().count();
    println!("\nTotal lines in system prompt: {}", line_count);
    println!("Total characters: {}", system_prompt.len());
}
