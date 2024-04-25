use std::{error::Error, fmt::Display, io};

use crate::lex::{Location, Token};
use lalrpop_util::ParseError;
type SchismParseError = ParseError<Location, Token, Token>;

#[derive(Debug)]
pub enum CompilationError {
    InvalidToken(usize, usize, SchismParseError),
    UnsupportedToken(usize, usize, Token, SchismParseError),
    Unexpected(usize, usize, Token, Vec<String>, SchismParseError),
    UnterminatedComment(Location),
    InvalidEscape(Location, String),
    LexicalError(Location, String),
    InvalidAtToken(Location, String),
    EOF(usize, SchismParseError),
    IO(io::Error),
    Descriptive(String),
}

impl Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LexicalError(l, m) => write!(f, "Lexical error {} at {}", m, l),
            Self::IO(i) => i.fmt(f),
            Self::Descriptive(d) => write!(f, "{}", d),
            CompilationError::InvalidToken(_, _, e) => e.fmt(f),
            CompilationError::UnsupportedToken(_, _, _, e) => e.fmt(f),
            CompilationError::Unexpected(_, _, _, _, e) => e.fmt(f),
            CompilationError::UnterminatedComment(l) => write!(f, "Unterminated comment at {}", l),
            CompilationError::InvalidEscape(c, loc) => {
                write!(f, "Invalid escape character '{}' at {}", c, loc)
            }
            CompilationError::EOF(_, e) => e.fmt(f),
            CompilationError::InvalidAtToken(l, t) => write!(f, "Invalid at-token {} at {}", t, l),
        }
    }
}

impl Error for CompilationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            CompilationError::LexicalError(_, _) => None,
            CompilationError::InvalidToken(_, _, ref e) => Some(e),
            CompilationError::UnsupportedToken(_, _, _, ref e) => Some(e),
            CompilationError::Unexpected(_, _, _, _, ref e) => Some(e),
            CompilationError::EOF(_, ref e) => Some(e),
            CompilationError::IO(ref e) => Some(e),
            CompilationError::Descriptive(_) => None,
            CompilationError::UnterminatedComment(_) => None,
            CompilationError::InvalidEscape(_, _) => None,
            CompilationError::InvalidAtToken(_, _) => None,
        }
    }
}
