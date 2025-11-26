use crate::app::App;
use once_cell::sync::Lazy;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style as SyntectStyle, ThemeSet};
use syntect::parsing::SyntaxSet;

// Global syntax highlighting resources
static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

// Convert syntect color to ratatui color
fn syntect_to_ratatui_color(color: syntect::highlighting::Color) -> Color {
    Color::Rgb(color.r, color.g, color.b)
}

// Parse a message and extract code blocks with syntax highlighting
fn parse_message_with_highlighting(text: &str, max_width: usize) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let mut in_code_block = false;
    let mut code_buffer: Vec<String> = Vec::new();
    let mut code_language = String::new();

    for line in text.lines() {
        if line.starts_with("```") {
            if in_code_block {
                // End of code block - highlight and flush
                let theme = &THEME_SET.themes["base16-ocean.dark"];
                let syntax = SYNTAX_SET
                    .find_syntax_by_token(&code_language)
                    .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());

                let mut highlighter = HighlightLines::new(syntax, theme);

                for code_line in &code_buffer {
                    let ranges: Vec<(SyntectStyle, &str)> = highlighter
                        .highlight_line(code_line, &SYNTAX_SET)
                        .unwrap_or_default();

                    let mut spans = Vec::new();
                    for (style, text) in ranges {
                        spans.push(Span::styled(
                            text.to_string(),
                            Style::default().fg(syntect_to_ratatui_color(style.foreground)),
                        ));
                    }
                    lines.push(Line::from(spans));
                }

                code_buffer.clear();
                in_code_block = false;
            } else {
                // Start of code block
                in_code_block = true;
                code_language = line.trim_start_matches("```").trim().to_string();
                if code_language.is_empty() {
                    code_language = "txt".to_string();
                }
            }
        } else if in_code_block {
            code_buffer.push(line.to_string());
        } else {
            // Regular text - wrap it
            for wrapped_line in wrap_text(line, max_width) {
                lines.push(Line::from(vec![Span::styled(
                    wrapped_line,
                    Style::default().fg(Color::White),
                )]));
            }
        }
    }

    // Handle unclosed code block
    if in_code_block && !code_buffer.is_empty() {
        for code_line in &code_buffer {
            lines.push(Line::from(vec![Span::styled(
                code_line.clone(),
                Style::default().fg(Color::Gray),
            )]));
        }
    }

    lines
}

// Enhanced helper function to wrap text to fit within a given width
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();

    // Handle empty text
    if text.trim().is_empty() {
        return vec![text.to_string()];
    }

    for line in text.lines() {
        if line.trim().is_empty() {
            // Handle empty lines by preserving them
            if !current_line.is_empty() {
                lines.push(current_line.clone());
                current_line.clear();
            }
            lines.push(String::new());
            continue;
        }

        let words: Vec<&str> = line.split_whitespace().collect();

        for word in words {
            // Handle words longer than max_width by breaking them
            if word.len() > max_width {
                if !current_line.is_empty() {
                    lines.push(current_line.clone());
                    current_line.clear();
                }
                // Break long word into chunks
                let mut remaining_word = word;
                while !remaining_word.is_empty() {
                    let chunk_size = std::cmp::min(max_width, remaining_word.len());
                    let chunk = &remaining_word[..chunk_size];
                    lines.push(chunk.to_string());
                    remaining_word = &remaining_word[chunk_size..];
                }
                continue;
            }

            // Check if word fits on current line
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
                // Word doesn't fit, start new line
                if !current_line.is_empty() {
                    lines.push(current_line.clone());
                    current_line.clear();
                }
                current_line.push_str(word);
            }
        }

        // Add the current line if it's not empty
        if !current_line.is_empty() {
            lines.push(current_line.clone());
            current_line.clear();
        }
    }

    // Handle case where we have pending content
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    // Ensure we always return at least the original text if no wrapping occurred
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

    // Add all conversation messages
    for message in &app.conversation {
        if let Some(content) = message.strip_prefix("User: ") {
            // Add user prefix
            conversation_lines.push(Line::from(vec![Span::styled(
                "User: ",
                Style::default().fg(Color::Blue).bold(),
            )]));

            // Parse and highlight the content
            let content_lines =
                parse_message_with_highlighting(content, max_width.saturating_sub(6));
            for line in content_lines {
                conversation_lines.push(line);
            }
        } else if let Some(content) = message.strip_prefix("Agent: ") {
            // Add agent prefix
            conversation_lines.push(Line::from(vec![Span::styled(
                "Agent: ",
                Style::default().fg(Color::Green).bold(),
            )]));

            // Parse and highlight the content
            let content_lines =
                parse_message_with_highlighting(content, max_width.saturating_sub(7));
            for line in content_lines {
                conversation_lines.push(line);
            }
        } else {
            let wrapped_message = wrap_text(message, max_width);
            for line in wrapped_message {
                conversation_lines.push(Line::from(vec![Span::styled(
                    line,
                    Style::default().fg(Color::Yellow),
                )]));
            }
        }

        // Add a blank line between messages for better readability
        conversation_lines.push(Line::from(vec![Span::styled("", Style::default())]));
    }

    // Add streaming message if currently streaming
    if app.is_streaming && !app.current_streaming_message.is_empty() {
        let streaming_message = format!("Agent: {}", app.current_streaming_message);
        let wrapped_streaming = wrap_text(&streaming_message, max_width.saturating_sub(7));

        for (i, line) in wrapped_streaming.iter().enumerate() {
            if i == 0 {
                conversation_lines.push(Line::from(vec![
                    Span::styled("Agent: ", Style::default().fg(Color::Green).bold()),
                    Span::styled(line.clone(), Style::default().fg(Color::Cyan).italic()),
                ]));
            } else {
                conversation_lines.push(Line::from(vec![
                    Span::styled("       ", Style::default().fg(Color::Green)), // Indent continuation
                    Span::styled(line.clone(), Style::default().fg(Color::Cyan).italic()),
                ]));
            }
        }

        // Add a blank line after streaming message
        conversation_lines.push(Line::from(vec![Span::styled("", Style::default())]));
    }

    // Calculate scroll position based on app state
    let visible_height = chunks[0].height.saturating_sub(2) as usize; // Account for borders
    let total_lines = conversation_lines.len();

    // Calculate max scroll position - ensure we can't scroll past the content
    let max_scroll = if total_lines > visible_height {
        total_lines.saturating_sub(visible_height)
    } else {
        0
    };

    // Clamp scroll position to valid range
    let scroll_position = if app.conversation_scroll_position == usize::MAX {
        // Auto-scroll to bottom when set to MAX
        max_scroll
    } else {
        app.conversation_scroll_position.min(max_scroll)
    };

    let conversation_block = Block::default().title("Conversation").borders(Borders::ALL);
    let conversation = Paragraph::new(conversation_lines.clone())
        .block(conversation_block)
        .scroll((scroll_position as u16, 0));
    f.render_widget(conversation, chunks[0]);

    // Add scrollbar for conversation
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    let mut scrollbar_state =
        ScrollbarState::new(conversation_lines.len()).position(scroll_position);
    f.render_stateful_widget(scrollbar, chunks[0], &mut scrollbar_state);

    // Note: scroll position is managed by the app state, not updated here

    // Tool logs section with wrapping
    let tool_logs_text = if app.tool_logs.is_empty() {
        "No tool executions yet...".to_string()
    } else {
        app.tool_logs.join("\n")
    };

    let tool_logs_lines: Vec<&str> = tool_logs_text.lines().collect();
    let tool_visible_height = chunks[1].height.saturating_sub(2) as usize; // Account for borders
    let tool_total_lines = tool_logs_lines.len();

    // Calculate max scroll for tool logs
    let tool_max_scroll = if tool_total_lines > tool_visible_height {
        tool_total_lines.saturating_sub(tool_visible_height)
    } else {
        0
    };

    let tool_scroll_position = app.tool_logs_scroll_position.min(tool_max_scroll);

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
    let mut tool_scrollbar_state =
        ScrollbarState::new(tool_logs_lines.len()).position(tool_scroll_position);
    f.render_stateful_widget(tool_scrollbar, chunks[1], &mut tool_scrollbar_state);

    // Note: tool logs scroll position is managed by the app state, not updated here

    // Enhanced input field with text wrapping
    let input_block = Block::default().title("Input").borders(Borders::ALL);
    let input_max_width = chunks[2].width.saturating_sub(4) as usize; // Account for borders and padding

    // Wrap the input text for better display
    let wrapped_input = if app.user_input.is_empty() {
        vec![String::new()]
    } else {
        wrap_text(&app.user_input, input_max_width)
    };

    let input_text = wrapped_input.join("\n");
    let input = Paragraph::new(input_text).block(input_block);
    f.render_widget(input, chunks[2]);

    let status_block = Block::default().title("Status").borders(Borders::ALL);
    let status = Paragraph::new(app.status_message.as_str()).block(status_block);
    f.render_widget(status, chunks[3]);
}
