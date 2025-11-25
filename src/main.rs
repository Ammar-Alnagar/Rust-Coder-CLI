mod agent;
mod app;
mod config;
mod llm;
mod ui;

use agent::Agent;
use app::App;
use config::Config;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::from_file("config.toml")?;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = Arc::new(Mutex::new(App::new()));
    let agent = Agent::new();
    let res = run_app(&mut terminal, app, agent, config).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

// Type alias for cleaner code
type AgentTaskResult = Result<(String, Vec<String>), Box<dyn std::error::Error + Send + Sync>>;
type AgentTask = task::JoinHandle<AgentTaskResult>;

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: Arc<Mutex<App>>,
    agent: Agent,
    config: Config,
) -> io::Result<()> {
    // Track if there's an ongoing agent task
    let mut current_agent_task: Option<AgentTask> = None;

    loop {
        // Always draw the UI first
        {
            let app_guard = app.lock().await;
            terminal.draw(|f| ui::ui(f, &app_guard))?;
        }

        // Check if the agent task has completed
        if let Some(ref mut task) = current_agent_task {
            if task.is_finished() {
                match task.await {
                    Ok(Ok((_response, tool_logs))) => {
                        let mut app_guard = app.lock().await;
                        // Add tool logs to the app
                        for log in tool_logs {
                            app_guard.add_tool_log(log);
                        }
                        // Response is already added to conversation in finish_streaming()
                        app_guard.status_message = "Done.".to_string();
                    }
                    Ok(Err(_e)) => {
                        let mut app_guard = app.lock().await;
                        // Error is handled in finish_streaming() or through agent error handling
                        app_guard.status_message = "Error.".to_string();
                    }
                    Err(_) => {
                        let mut app_guard = app.lock().await;
                        app_guard.status_message = "Task panicked.".to_string();
                    }
                }
                current_agent_task = None;
            }
        }

        // Check for events with a timeout - this allows UI to update during streaming
        if let Ok(event_available) = event::poll(Duration::from_millis(50)) {
            if event_available {
                if let Ok(Event::Key(key)) = event::read() {
                    // Only process key events (ignore mouse events, resize events, etc.)
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                // Ctrl+C to quit
                                return Ok(());
                            }
                            KeyCode::Char(c) => {
                                // Handle character input
                                let mut app_guard = app.lock().await;
                                app_guard.user_input.push(c);
                            }
                            KeyCode::Backspace => {
                                let mut app_guard = app.lock().await;
                                app_guard.user_input.pop();
                            }
                            KeyCode::Up => {
                                // Scroll conversation up
                                let mut app_guard = app.lock().await;
                                app_guard.scroll_conversation_up();
                            }
                            KeyCode::Down => {
                                // Scroll conversation down
                                let mut app_guard = app.lock().await;
                                app_guard.scroll_conversation_down();
                            }
                            KeyCode::PageUp => {
                                // Scroll conversation up by page
                                let mut app_guard = app.lock().await;
                                let page_size = 10; // Approximate lines per page
                                for _ in 0..page_size {
                                    app_guard.scroll_conversation_up();
                                }
                            }
                            KeyCode::PageDown => {
                                // Scroll conversation down by page
                                let mut app_guard = app.lock().await;
                                let page_size = 10; // Approximate lines per page
                                for _ in 0..page_size {
                                    app_guard.scroll_conversation_down();
                                }
                            }
                            KeyCode::Home => {
                                // Scroll to top of conversation
                                let mut app_guard = app.lock().await;
                                app_guard.scroll_conversation_to_top();
                            }
                            KeyCode::End => {
                                // Scroll to bottom of conversation
                                let mut app_guard = app.lock().await;
                                app_guard.scroll_conversation_to_bottom();
                            }
                            KeyCode::Enter => {
                                // Don't process new input if there's already an agent task running
                                if current_agent_task.is_some() {
                                    continue;
                                }

                                let user_input = {
                                    let mut app_guard = app.lock().await;
                                    app_guard.user_input.drain(..).collect::<String>()
                                };

                                // Check for quit command
                                if user_input.trim() == "/quit" {
                                    // Display usage summary before quitting
                                    let summary = {
                                        let app_guard = app.lock().await;
                                        app_guard.get_usage_summary()
                                    };
                                    {
                                        let mut app_guard = app.lock().await;
                                        app_guard.conversation.push(format!("System: {}", summary));
                                        terminal.draw(|f| ui::ui(f, &app_guard))?;
                                    }

                                    // Give user a moment to see the summary
                                    std::thread::sleep(std::time::Duration::from_secs(2));
                                    return Ok(());
                                }

                                // Check for stats command
                                if user_input.trim() == "/stats" {
                                    let summary = {
                                        let app_guard = app.lock().await;
                                        app_guard.get_usage_summary()
                                    };
                                    let mut app_guard = app.lock().await;
                                    app_guard.conversation.push(format!("System: {}", summary));
                                    continue;
                                }

                                {
                                    let mut app_guard = app.lock().await;
                                    app_guard.conversation.push(format!("User: {}", user_input));
                                    app_guard.status_message =
                                        "🤔 Thinking... (streaming response will appear live)"
                                            .to_string();
                                    terminal.draw(|f| ui::ui(f, &app_guard))?;
                                }

                                // Spawn the agent task in the background so the UI can continue updating
                                let mut agent_clone = agent.clone();
                                let config_clone = config.clone();
                                let user_input_clone = user_input.clone();
                                let app_clone = Arc::clone(&app);
                                current_agent_task = Some(task::spawn(async move {
                                    // Run the agent with access to the shared app state
                                    // The agent will handle its own locking/unlocking to allow UI updates
                                    let result = agent_clone
                                        .run(&config_clone.llm, user_input_clone, app_clone)
                                        .await;
                                    result
                                }));
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
