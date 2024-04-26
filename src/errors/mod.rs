// Copyright 2024 Mark C. Chu-Carroll
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{error::Error, fmt::Display, io};

use crate::lex::{Location, Token};
use lalrpop_util::ParseError;
use crate::ast::ModulePath;

#[derive(Debug)]
pub enum CompilationError {
    InvalidToken(Location),
    UnrecognizedToken(Location, Token, Vec<String>),
    UnexpectedToken(Location, Token),
    UnterminatedComment(Location),
    InvalidEscape(Location, String),
    LexicalError(Location, String),
    ModuleNotFoundError(ModulePath),
    InvalidAtToken(Location, String),
    EOF{ location: Location, expected: Vec<String> },
    IO(io::Error),
    Descriptive(String),
}

impl CompilationError {
    fn copy(&self) -> CompilationError {
        match self {
            CompilationError::InvalidToken(l) => CompilationError::InvalidToken(l.clone()),
            CompilationError::UnrecognizedToken(l, t, v) => CompilationError::UnrecognizedToken(l.clone(), t.clone(), v.clone()),
            CompilationError::UnexpectedToken(l, t) => CompilationError::UnexpectedToken(l.clone(), t.clone()),
            CompilationError::UnterminatedComment(l) => CompilationError::UnterminatedComment(l.clone()),
            CompilationError::InvalidEscape(l, s) => CompilationError::InvalidEscape(l.clone(), s.clone()),
            CompilationError::LexicalError(l, s) => CompilationError::LexicalError(l.clone(), s.clone()),
            CompilationError::ModuleNotFoundError(m) =>CompilationError::ModuleNotFoundError(m.clone()),
            CompilationError::InvalidAtToken(l, s) => CompilationError::InvalidAtToken(l.clone(), s.clone()),
            CompilationError::EOF { location, expected } => CompilationError::EOF { location: location.clone(), expected: expected.clone() },
            CompilationError::IO(i) => CompilationError::Descriptive(i.to_string()),
            CompilationError::Descriptive(s) => CompilationError::Descriptive(s.clone())
        }

    }
}

impl From<ParseError<Location, Token, CompilationError>> for CompilationError {
    fn from(value: ParseError<Location, Token, CompilationError>) -> Self {
        match &value {
            ParseError::InvalidToken { location } => CompilationError::InvalidToken(location.to_owned()),
            ParseError::UnrecognizedEof { location, expected } => CompilationError::EOF{location: location.to_owned(),
                expected: expected.to_owned()},
            ParseError::UnrecognizedToken { token: (start, t, _), expected } =>
                CompilationError::UnrecognizedToken(start.to_owned(), t.to_owned(), expected.to_owned()),
            ParseError::ExtraToken { token: (start, t, _) } =>
                CompilationError::UnexpectedToken(start.clone(), t.clone()),
            ParseError::User { error } =>
              error.copy()
        }
    }
}

impl From<io::Error> for CompilationError {
    fn from(value: io::Error) -> Self {
        CompilationError::IO(value)
    }
}
impl Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LexicalError(l, m) => write!(f, "Lexical error {} at {}", m, l),
            Self::IO(i) => i.fmt(f),
            Self::Descriptive(d) => write!(f, "{}", d),
            CompilationError::InvalidToken(loc) => write!(f, "Invalid token at {}", loc),
            CompilationError::UnrecognizedToken(loc, tok, expected) =>
                write!(f, "Unrecognized token {un} while expecting one of {exp} at {loc}",
                    un=tok, exp=expected.join(", "), loc=loc),
            CompilationError::UnexpectedToken(loc, tok) => write!(f, "Unexpected token {} at {}", tok, loc),
            CompilationError::UnterminatedComment(l) => write!(f, "Unterminated comment at {}", l),
            CompilationError::InvalidEscape(c, loc) => {
                write!(f, "Invalid escape character '{}' at {}", c, loc)
            }
            CompilationError::EOF { location, expected } =>
                write!(f, "Reached EOF while expecting {exp} at {loc}",
                       exp=expected.join(", "), loc=location),
            CompilationError::InvalidAtToken(l, t) => write!(f, "Invalid at-token {} at {}", t, l),
            CompilationError::ModuleNotFoundError(path) => write!(f, "Module {} not found", path)
        }
    }
}

impl Error for CompilationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CompilationError::LexicalError(_, _) => None,
            CompilationError::InvalidToken(_) => None,
            CompilationError::UnrecognizedToken(_, _, _) => None,
            CompilationError::UnexpectedToken(_, _) => None,
            CompilationError::EOF{..} => None,
            CompilationError::IO(ref e) => Some(e),
            CompilationError::Descriptive(_) => None,
            CompilationError::UnterminatedComment(_) => None,
            CompilationError::InvalidEscape(_, _) => None,
            CompilationError::InvalidAtToken(_, _) => None,
            CompilationError::ModuleNotFoundError(_) => None
        }
    }
}

