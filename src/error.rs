pub enum Error {
    LexicalError {
        line: usize,
        column: usize,
        message: String,
    },
    ParseError {
        line: usize,
        column: usize,
        message: String,
    },
}
