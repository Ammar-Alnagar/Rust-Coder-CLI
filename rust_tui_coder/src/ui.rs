use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
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

    let conversation_text: Vec<String> = app.conversation.iter().map(|m| format!("{}: {}", m.role, m.content)).collect();
    let conversation_block = Block::default().title("Conversation").borders(Borders::ALL);
    let conversation = Paragraph::new(conversation_text.join("\n"))
        .block(conversation_block);
    f.render_widget(conversation, chunks[0]);

    let input_block = Block::default().title("Input").borders(Borders::ALL);
    let input = Paragraph::new(app.user_input.as_str())
        .block(input_block);
    f.render_widget(input, chunks[1]);

    let status_block = Block::default().title("Status").borders(Borders::ALL);
    let status = Paragraph::new(app.status_message.as_str())
        .block(status_block);
    f.render_widget(status, chunks[2]);
}
