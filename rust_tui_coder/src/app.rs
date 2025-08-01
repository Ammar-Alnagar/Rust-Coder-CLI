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

    pub fn start_tool_execution(&mut self, tool_name: &str) {
        self.is_executing_tool = true;
        self.current_tool = tool_name.to_string();
        self.status_message = format!("Executing: {}", tool_name);
    }

    pub fn end_tool_execution(&mut self) {
        self.is_executing_tool = false;
        self.current_tool.clear();
        self.status_message = "Done.".to_string();
    }
}
