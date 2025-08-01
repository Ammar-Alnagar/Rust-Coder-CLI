pub struct App {
    pub user_input: String,
    pub conversation: Vec<String>,
    pub status_message: String,
    pub tool_logs: Vec<String>,
    pub is_executing_tool: bool,
    pub current_tool: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            user_input: String::new(),
            conversation: Vec::new(),
            status_message: "Type in /quit to exit".to_string(),
            tool_logs: Vec::new(),
            is_executing_tool: false,
            current_tool: String::new(),
        }
    }

    pub fn add_tool_log(&mut self, log: String) {
        self.tool_logs.push(log);
    }
}
