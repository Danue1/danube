#[derive(Debug)]
pub struct SyntaxError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let line = self.line + 1;
        let column = self.column + 1;
        let message = self.message.as_str();
        write!(f, "Syntax error at line {line}, column {column}: {message}",)
    }
}

impl std::error::Error for SyntaxError {
    //
}
