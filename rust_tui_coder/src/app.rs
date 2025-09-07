pub struct App {
    pub user_input: String,
    pub conversation: Vec<String>,
    pub status_message: String,
    pub tool_logs: Vec<String>,
    pub is_executing_tool: bool,
    pub current_tool: String,
    // Usage tracking
    pub session_start_time: std::time::Instant,
    pub tokens_used: u64,
    pub total_requests: u64,
    pub total_tools_executed: u64,
}

impl App {
    pub fn new() -> Self {
        Self {
            user_input: String::new(),
            conversation: Vec::new(),
            status_message: "Commands: /quit (exit), /stats (usage stats)".to_string(),
            tool_logs: Vec::new(),
            is_executing_tool: false,
            current_tool: String::new(),
            // Initialize usage tracking
            session_start_time: std::time::Instant::now(),
            tokens_used: 0,
            total_requests: 0,
            total_tools_executed: 0,
        }
    }

    pub fn add_tool_log(&mut self, log: String) {
        self.tool_logs.push(log);
    }

    // Usage tracking methods
    pub fn increment_tokens(&mut self, tokens: u64) {
        self.tokens_used += tokens;
    }

    pub fn increment_requests(&mut self) {
        self.total_requests += 1;
    }

    pub fn increment_tools_executed(&mut self) {
        self.total_tools_executed += 1;
    }

    pub fn get_session_duration(&self) -> std::time::Duration {
        self.session_start_time.elapsed()
    }

    pub fn get_usage_summary(&self) -> String {
        let duration = self.get_session_duration();
        let hours = duration.as_secs() / 3600;
        let minutes = (duration.as_secs() % 3600) / 60;
        let seconds = duration.as_secs() % 60;

        format!(
            "Session Summary:\n\
             • Duration: {:02}:{:02}:{:02}\n\
             • Tokens Used: {}\n\
             • LLM Requests: {}\n\
             • Tools Executed: {}\n\
             • Average Tokens/Request: {}",
            hours,
            minutes,
            seconds,
            self.tokens_used,
            self.total_requests,
            self.total_tools_executed,
            if self.total_requests > 0 {
                self.tokens_used / self.total_requests
            } else {
                0
            }
        )
    }
}
