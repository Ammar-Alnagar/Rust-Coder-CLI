// Test the wrap_text function
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();

    if text.trim().is_empty() {
        return vec![text.to_string()];
    }

    for line in text.lines() {
        if line.trim().is_empty() {
            if !current_line.is_empty() {
                lines.push(current_line.clone());
                current_line.clear();
            }
            lines.push(String::new());
            continue;
        }

        let words: Vec<&str> = line.split_whitespace().collect();

        for word in words {
            if word.len() > max_width {
                if !current_line.is_empty() {
                    lines.push(current_line.clone());
                    current_line.clear();
                }
                let mut remaining_word = word;
                while !remaining_word.is_empty() {
                    let chunk_size = std::cmp::min(max_width, remaining_word.len());
                    let chunk = &remaining_word[..chunk_size];
                    lines.push(chunk.to_string());
                    remaining_word = &remaining_word[chunk_size..];
                }
                continue;
            }

            let space_needed = if current_line.is_empty() {
                word.len()
            } else {
                current_line.len() + 1 + word.len()
            };

            if space_needed <= max_width {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            } else {
                if !current_line.is_empty() {
                    lines.push(current_line.clone());
                    current_line.clear();
                }
                current_line.push_str(word);
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line.clone());
            current_line.clear();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(text.to_string());
    }

    lines
}

fn main() {
    let test_text = "Here are the tools:\n1. CREATE_PLAN - Create a plan for complex tasks\n2. UPDATE_PLAN - Mark a step as completed in the plan";
    
    println!("Original text:");
    println!("{}", test_text);
    println!("\nWrapped at 40 chars:");
    let wrapped = wrap_text(test_text, 40);
    for (i, line) in wrapped.iter().enumerate() {
        println!("{}: '{}'", i, line);
    }
    
    // Test garbled-looking text
    let garbled = "Here the tools1 CREATE - a plan complex";
    println!("\nGarbled text:");
    println!("{}", garbled);
    println!("\nWrapped at 40 chars:");
    let wrapped2 = wrap_text(garbled, 40);
    for (i, line) in wrapped2.iter().enumerate() {
        println!("{}: '{}'", i, line);
    }
}
