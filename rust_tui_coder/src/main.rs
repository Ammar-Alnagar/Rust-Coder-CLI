mod config;
mod llm;
mod agent;
mod app;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;
use app::App;
use agent::Agent;
use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::from_file("config.toml")?;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let agent = Agent::new();
    let res = run_app(&mut terminal, app, agent, config).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    mut agent: Agent,
    config: Config,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    app.user_input.push(c);
                }
                KeyCode::Backspace => {
                    app.user_input.pop();
                }
                KeyCode::Enter => {
                    let user_input = app.user_input.drain(..).collect::<String>();
                    
                    // Check for quit command
                    if user_input.trim() == "/quit" {
                        return Ok(());
                    }
                    
                    app.conversation.push(format!("User: {}", user_input));

                    app.status_message = "Thinking...".to_string();
                    terminal.draw(|f| ui::ui(f, &app))?;

                    match agent.run(&config.llm, user_input).await {
                        Ok(response) => {
                            app.conversation.push(format!("Agent: {}", response));
                            app.status_message = "Done.".to_string();
                        }
                        Err(e) => {
                            app.conversation.push(format!("Error: {}", e));
                            app.status_message = "Error.".to_string();
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
