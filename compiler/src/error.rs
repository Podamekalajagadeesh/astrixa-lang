/// Represents a compilation error with location and helpful information
#[derive(Debug, Clone)]
pub struct CompileError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub help: Option<String>,
}

impl CompileError {
    /// Create a new compile error
    pub fn new(msg: &str, line: usize, column: usize) -> Self {
        Self {
            message: msg.to_string(),
            line,
            column,
            help: None,
        }
    }

    /// Add helpful context to the error
    pub fn help(mut self, text: &str) -> Self {
        self.help = Some(text.to_string());
        self
    }
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {}\n → line {}, column {}",
            self.message, self.line, self.column
        )
    }
}
