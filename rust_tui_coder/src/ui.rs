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
                Constraint::Percentage(80), // For conversation
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

    let input_block = Block::default().title("Input").borders(Borders::ALL);
    let input = Paragraph::new(app.user_input.as_str())
        .block(input_block);
    f.render_widget(input, chunks[1]);

    let status_block = Block::default().title("Status").borders(Borders::ALL);
    let status = Paragraph::new(app.status_message.as_str())
        .block(status_block);
    f.render_widget(status, chunks[2]);
}
