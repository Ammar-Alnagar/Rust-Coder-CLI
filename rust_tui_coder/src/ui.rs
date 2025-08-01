use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    style::{Color, Style, Stylize},
    text::{Span, Line},
    Frame,
};
use crate::app::App;

// Helper function to wrap text to fit within a given width
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for word in words {
        if current_line.len() + word.len() + 1 <= max_width {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            if !current_line.is_empty() {
                lines.push(current_line.clone());
            }
            current_line = word.to_string();
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

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(60), // For conversation
                Constraint::Percentage(20), // For tool logs
                Constraint::Percentage(10), // For user input
                Constraint::Percentage(10), // For status
            ]
            .as_ref(),
        )
        .split(f.size());

    // Create conversation text with proper formatting and colors
    let mut conversation_lines = Vec::new();
    let max_width = chunks[0].width.saturating_sub(4) as usize; // Account for borders and padding
    
    for message in &app.conversation {
        if message.starts_with("User: ") {
            let content = &message[6..]; // Remove "User: " prefix
            let wrapped_content = wrap_text(content, max_width.saturating_sub(6)); // Account for "User: " prefix
            
            for (i, line) in wrapped_content.iter().enumerate() {
                if i == 0 {
                    conversation_lines.push(Line::from(vec![
                        Span::styled("User: ", Style::default().fg(Color::Blue).bold()),
                        Span::styled(line.clone(), Style::default().fg(Color::White)),
                    ]));
                } else {
                    conversation_lines.push(Line::from(vec![
                        Span::styled("      ", Style::default().fg(Color::Blue)), // Indent continuation
                        Span::styled(line.clone(), Style::default().fg(Color::White)),
                    ]));
                }
            }
        } else if message.starts_with("Agent: ") {
            let content = &message[7..]; // Remove "Agent: " prefix
            let wrapped_content = wrap_text(content, max_width.saturating_sub(7)); // Account for "Agent: " prefix
            
            for (i, line) in wrapped_content.iter().enumerate() {
                if i == 0 {
                    conversation_lines.push(Line::from(vec![
                        Span::styled("Agent: ", Style::default().fg(Color::Green).bold()),
                        Span::styled(line.clone(), Style::default().fg(Color::White)),
                    ]));
                } else {
                    conversation_lines.push(Line::from(vec![
                        Span::styled("       ", Style::default().fg(Color::Green)), // Indent continuation
                        Span::styled(line.clone(), Style::default().fg(Color::White)),
                    ]));
                }
            }
        } else {
            let wrapped_message = wrap_text(message, max_width);
            for line in wrapped_message {
                conversation_lines.push(Line::from(vec![
                    Span::styled(line, Style::default().fg(Color::Yellow)),
                ]));
            }
        }
        
        // Add a blank line between messages for better readability
        conversation_lines.push(Line::from(vec![
            Span::styled("", Style::default()),
        ]));
    }
    
    // Calculate scroll position to show the latest messages
    let max_scroll = conversation_lines.len().saturating_sub(chunks[0].height as usize);
    let scroll_position = max_scroll;

    let conversation_block = Block::default().title("Conversation").borders(Borders::ALL);
    let conversation = Paragraph::new(conversation_lines.clone())
        .block(conversation_block)
        .scroll((scroll_position as u16, 0));
    f.render_widget(conversation, chunks[0]);

    // Add scrollbar for conversation
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    let mut scrollbar_state = ScrollbarState::new(conversation_lines.len())
        .position(scroll_position);
    f.render_stateful_widget(scrollbar, chunks[0], &mut scrollbar_state);

    // Tool logs section with wrapping
    let tool_logs_text = if app.tool_logs.is_empty() {
        "No tool executions yet...".to_string()
    } else {
        app.tool_logs.join("\n")
    };
    
    let tool_logs_lines: Vec<&str> = tool_logs_text.lines().collect();
    let tool_max_scroll = tool_logs_lines.len().saturating_sub(chunks[1].height as usize);
    let tool_scroll_position = tool_max_scroll;

    let tool_logs_block = Block::default()
        .title(if app.is_executing_tool {
            format!("Tool Logs - Currently executing: {}", app.current_tool)
        } else {
            "Tool Logs".to_string()
        })
        .borders(Borders::ALL);
    let tool_logs = Paragraph::new(tool_logs_text.clone())
        .block(tool_logs_block)
        .scroll((tool_scroll_position as u16, 0));
    f.render_widget(tool_logs, chunks[1]);

    // Add scrollbar for tool logs
    let tool_scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    let mut tool_scrollbar_state = ScrollbarState::new(tool_logs_lines.len())
        .position(tool_scroll_position);
    f.render_stateful_widget(tool_scrollbar, chunks[1], &mut tool_scrollbar_state);

    let input_block = Block::default().title("Input").borders(Borders::ALL);
    let input = Paragraph::new(app.user_input.as_str())
        .block(input_block);
    f.render_widget(input, chunks[2]);

    let status_block = Block::default().title("Status").borders(Borders::ALL);
    let status = Paragraph::new(app.status_message.as_str())
        .block(status_block);
    f.render_widget(status, chunks[3]);
}
