use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};
use crate::app::App;

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

    // Create conversation text with proper formatting
    let conversation_text = app.conversation.join("\n\n");
    
    // Calculate scroll position to show the latest messages
    let conversation_lines: Vec<&str> = conversation_text.lines().collect();
    let max_scroll = conversation_lines.len().saturating_sub(chunks[0].height as usize);
    let scroll_position = max_scroll;

    let conversation_block = Block::default().title("Conversation").borders(Borders::ALL);
    let conversation = Paragraph::new(conversation_text.clone())
        .block(conversation_block)
        .scroll((scroll_position as u16, 0));
    f.render_widget(conversation, chunks[0]);

    // Add scrollbar for conversation
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    let mut scrollbar_state = ScrollbarState::new(conversation_lines.len())
        .position(scroll_position);
    f.render_stateful_widget(scrollbar, chunks[0], &mut scrollbar_state);

    // Tool logs section
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
