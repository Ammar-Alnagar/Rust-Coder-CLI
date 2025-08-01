pub struct App {
    pub user_input: String,
    pub conversation: Vec<String>,
    pub status_message: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            user_input: String::new(),
            conversation: Vec::new(),
            status_message: "Press 'q' to quit".to_string(),
        }
    }
}
