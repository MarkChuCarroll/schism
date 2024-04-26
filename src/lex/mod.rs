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

use line_col::LineColLookup;
use std::{collections::HashMap, fmt::Display, str::CharIndices};
use unicode_categories::UnicodeCategories;

use crate::ast::*;
use crate::errors::CompilationError;

mod location;
mod token;
pub use location::Location;
pub use token::Token;
/// An extension trait providing tests of a couple of
/// character categories that are useful for the parser.
trait CharacterCategories {
    fn is_lname_start_char(&self) -> bool;
    fn is_id_char(&self) -> bool;
    fn is_syntax_char(&self) -> bool;
    fn is_uname_start_char(&self) -> bool;
}

impl CharacterCategories for char {
    fn is_uname_start_char(&self) -> bool {
        self.is_alphabetic() && self.is_uppercase()
    }

    fn is_lname_start_char(&self) -> bool {
        return !self.is_syntax_char()
            && !self.is_whitespace()
            && ((self.is_alphabetic() && self.is_lowercase())
                || self.is_symbol()
                || self.is_punctuation_connector()
                || self.is_punctuation_dash()
                || self.is_punctuation_other());
    }

    fn is_id_char(&self) -> bool {
        return !self.is_syntax_char()
            && !self.is_whitespace()
            && (self.is_alphabetic()
                || self.is_punctuation_connector()
                || self.is_punctuation_dash()
                || self.is_punctuation_other()
                || self.is_symbol()
                || self.is_number());
    }

    fn is_syntax_char(&self) -> bool {
        match self {
            '\'' | '.' | '"' | '$' | '@' | '|' | '(' | ')' | '{' | '}' | '[' | ']' => true,
            _ => false,
        }
    }
}

pub struct Scanner<'input> {
    index: LineColLookup<'input>,
    chars: std::iter::Peekable<CharIndices<'input>>,
    input: &'input str,
    current: Option<(usize, char)>,
    next: Option<(usize, char)>,
    reserved: HashMap<String, Token>,
}

impl<'input> Scanner<'input> {
    pub fn new(input: &'input str) -> Scanner<'input> {
        let mut scanner = Scanner {
            index: LineColLookup::new(input),
            chars: input.char_indices().peekable(),
            current: None,
            next: None,
            input,
            reserved: HashMap::from([
                ("--".to_string(), Token::DASHDASH),
                ("->".to_string(), Token::ARROW),
                ("action".to_string(), Token::ACTION),
                ("@action".to_string(), Token::ATACTION),
                ("composes".to_string(), Token::COMPOSES),
                ("cond".to_string(), Token::COND),
                ("@cond".to_string(), Token::ATCOND),
                ("do".to_string(), Token::DO),
                ("end".to_string(), Token::END),
                ("else".to_string(), Token::ELSE),
                ("exit".to_string(), Token::EXIT),
                ("for".to_string(), Token::FOR),
                ("@for".to_string(), Token::ATFOR),
                ("fun".to_string(), Token::FUNCTION),
                ("@fun".to_string(), Token::ATFUNCTION),
                ("init".to_string(), Token::INIT),
                ("is".to_string(), Token::IS),
                ("local".to_string(), Token::LOCAL),
                ("loop".to_string(), Token::LOOP),
                ("@loop".to_string(), Token::ATLOOP),
                ("meth".to_string(), Token::METHOD),
                ("@meth".to_string(), Token::ATMETHOD),
                ("next".to_string(), Token::NEXT),
                ("new".to_string(), Token::NEW),
                ("obj".to_string(), Token::OBJ),
                ("@obj".to_string(), Token::ATOBJ),
                ("sig".to_string(), Token::SIG),
                ("@sig".to_string(), Token::ATSIG),
                ("slot".to_string(), Token::SLOT),
                ("@slot".to_string(), Token::ATSLOT),
                ("use".to_string(), Token::USE),
                ("var".to_string(), Token::VAR),
                ("@var".to_string(), Token::ATVAR),
                ("!".to_string(), Token::BANG),
            ]),
        };
        scanner.advance();
        return scanner;
    }

    /// Convert a position within the input string to
    /// a (line, column) pair.
    ///
    /// Note that this assumes that the position was returned
    /// by the scanner as the location of a token. It will panic
    /// if you give it an index beyond the end of the input.
    pub fn line_and_col(&self, pos: usize) -> Location {
        let (l, c) = self.index.get(pos);
        Location { line: l, column: c }
    }

    fn advance(&mut self) {
        self.current = self.chars.next();
        self.next = match self.chars.peek() {
            Some(u) => Some(*u),
            None => None,
        }
    }

    fn ident_or_keyword(
        &self,
        token_str: String,
        start: usize,
        end: usize,
    ) -> Option<ScannerResult> {
        match self.reserved.get(&token_str) {
            Some(t) => self.good_token(t.clone(), start, end),
            None => self.good_token(Token::LName(LowerName(token_str)), start, end),
        }
    }

    fn validate_at_keyword(
        &self,
        token_str: &str,
        start: usize,
        end: usize,
    ) -> Option<ScannerResult> {
        match self.reserved.get(token_str) {
            Some(t) => self.good_token(t.clone(), start, end),
            None => Some(Err(CompilationError::InvalidAtToken(
                self.line_and_col(start),
                token_str.to_string(),
            ))),
        }
    }

    fn validate_typevar(&self, token_str: &str, start: usize, end: usize) -> Option<ScannerResult> {
        if token_str.len() < 2 {
            Some(Err(CompilationError::LexicalError(
                self.line_and_col(start),
                "Type var must have at least one letter after its sigil".to_string(),
            )))
        } else {
            self.good_token(
                Token::TVName(TypeVarName(token_str.to_string())),
                start,
                end,
            )
        }
    }

    fn validate_contextvar(
        &self,
        token_str: &str,
        start: usize,
        end: usize,
    ) -> Option<ScannerResult> {
        if token_str.len() < 2 {
            Some(Err(CompilationError::LexicalError(
                self.line_and_col(start),
                "Context var must have at least one letter after its sigil".to_string(),
            )))
        } else {
            self.good_token(Token::CName(ContextName(token_str.to_string())), start, end)
        }
    }
}

pub type ScannerResult<'input> = Result<(Location, Token, Location), CompilationError>;

impl<'input> Iterator for Scanner<'input> {
    type Item = ScannerResult<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        return self.scan_token();
    }
}

/// This impl block contains the meat of the scanner.
///
/// It's a relatively straightforward scanner. The easiest way to think
/// of it is that it's an FSM, where each state in the FSM is
/// a scan function, and we return either a token or an error when
/// we reach a final state.
///
/// So, for example, the scanning process for a float literal:
/// - Enter the "scan_number" state. Any numeric character stays in
///    "scan_number". A "." switches to "scan_float"; any non-numeric
///    is a terminal which returns an integer token.
/// - In the scan_float state, you consume the ".", and then again
///   stay in the state for any numeric character. If you see an "e"
///   (for exponent), then you switch to "scan_float_exponent".
/// - Etc.
///
/// Each time that you switch states in the above explanation, you
/// just call the new state function in the scanner code.
impl<'input> Scanner<'input> {
    fn good_token(&self, t: Token, start: usize, end: usize) -> Option<ScannerResult<'input>> {
        Some(Ok((self.line_and_col(start), t, self.line_and_col(end))))
    }

    pub fn scan_token(&mut self) -> Option<ScannerResult<'input>> {
        loop {
            match self.current {
                // Skip WS
                Some((_, ' ')) | Some((_, '\n')) | Some((_, '\t')) => {
                    self.advance();
                    continue;
                }
                // Unambiguous Single character tokens
                Some((pos, '(')) => {
                    self.advance();
                    return self.good_token(Token::LPAREN, pos, pos + 1);
                }
                Some((pos, ')')) => {
                    self.advance();
                    return self.good_token(Token::RPAREN, pos, pos + 1);
                }

                Some((pos, '{')) => {
                    self.advance();
                    return self.good_token(Token::LCURLY, pos, pos + 1);
                }
                Some((pos, '}')) => {
                    self.advance();
                    return self.good_token(Token::RCURLY, pos, pos + 1);
                }
                Some((pos, ']')) => {
                    self.advance();
                    return self.good_token(Token::RBRACK, pos, pos + 1);
                }
                Some((pos, '.')) => {
                    self.advance();
                    return self.good_token(Token::DOT, pos, pos + 1);
                }
                Some((pos, ',')) => {
                    self.advance();
                    return self.good_token(Token::COMMA, pos, pos + 1);
                }
                // Then look at possible one-character tokens that could
                // also be the first character of a multichar token.
                Some((pos, '[')) => {
                    self.advance();
                    match self.current {
                        Some((_, '|')) => {
                            self.advance();
                            return self.good_token(Token::LBRACKBAR, pos, pos + 2);
                        }
                        _ => return self.good_token(Token::LBRACK, pos, pos + 1),
                    }
                }
                Some((pos, '|')) => match self.next {
                    Some((_, ']')) => {
                        self.advance();
                        self.advance();
                        return self.good_token(Token::RBRACKBAR, pos, pos + 2);
                    }
                    _ => return self.scan_ident_or_keyword(pos),
                },
                // Multicharacter tokens/elements with a distinguishing leader
                Some((start, '/')) => match self.next {
                    Some((_, '/')) => {
                        self.skip_to_end_of_line();
                        continue;
                    }
                    Some((_, '*')) => {
                        let skip = self.skip_comment(start);
                        if skip.is_err() {
                            return Some(Err(skip.unwrap_err()));
                        }
                        continue;
                    }
                    _ => return self.scan_ident_or_keyword(start),
                },
                Some((p, '@')) => return self.scan_at_ident(p),
                Some((p, '$')) => return self.scan_contextvar(p),
                Some((p, '`')) => return self.scan_typevar(p),
                Some((pos, c)) if c.is_number_decimal_digit() => return self.scan_number(pos),
                Some((pos, '-')) => match self.next {
                    Some((_, c)) if c.is_number_decimal_digit() => return self.scan_number(pos),
                    _ => return self.scan_ident_or_keyword(pos),
                },
                Some((pos, '"')) => return self.scan_string(pos),
                Some((pos, '\'')) => return self.scan_char_literal(pos),

                Some((pos, ':')) => {
                    self.advance();
                    return self.good_token(Token::COLON, pos, pos + 1);
                }
                Some((pos, c)) => {
                    if c.is_lname_start_char() {
                        return self.scan_ident_or_keyword(pos);
                    } else if c.is_uppercase() {
                        return self.scan_upper_ident(pos);
                    } else if c.is_number_decimal_digit() {
                        return self.scan_number(pos);
                    } else {
                        return Some(Err(CompilationError::LexicalError(
                            self.line_and_col(pos),
                            "Invalid char".to_string(),
                        )));
                    }
                }
                None => return None,
            }
        }
    }

    fn skip_comment(&mut self, start: usize) -> Result<(), CompilationError> {
        self.advance();
        loop {
            match self.current {
                Some((_, '*')) => {
                    self.advance();
                    match self.current {
                        Some((_, '/')) => {
                            self.advance();
                            return Ok(());
                        }
                        _ => continue,
                    }
                }
                Some(_) => {
                    self.advance();
                }
                None => {
                    let loc = self.line_and_col(start);
                    return Err(CompilationError::UnterminatedComment(loc));
                }
            }
        }
    }

    /// Scan a numeric literal.
    fn scan_number(&mut self, start: usize) -> Option<ScannerResult<'input>> {
        let mut count = 0;
        if let Some((_, c)) = self.current {
            if c == '-' {
                self.advance();
            }
        }
        loop {
            if let Some((i, c)) = self.current {
                count = count + 1;
                if c.is_ascii_digit() {
                    self.advance();
                    continue;
                } else if c == '.' {
                    self.advance();
                    return self.scan_float(start);
                } else {
                    return self.good_token(
                        Token::INTLIT(self.input[start..i].parse::<i64>().unwrap()),
                        start,
                        i,
                    );
                }
            } else {
                return self.good_token(
                    Token::INTLIT(self.input[start..(start + count)].parse::<i64>().unwrap()),
                    start,
                    start + count,
                );
            }
        }
    }

    /// Scan the fractional part of a floating point literal.
    /// This state is only entered from scan_number, and returns a token
    /// containing everything matched by both scan_number and this state.
    fn scan_float(&mut self, start: usize) -> Option<ScannerResult<'input>> {
        loop {
            if let Some((i, c)) = self.current {
                if c.is_ascii_digit() {
                    self.advance();
                    continue;
                } else if c == 'e' {
                    self.advance();
                    return self.scan_float_exponent(start);
                } else {
                    return self.good_token(
                        Token::FLOATLIT(self.input[start..i].to_string()),
                        start,
                        i,
                    );
                }
            }
        }
    }

    /// Scan the exponent part of a floating point literal.
    /// This state is only entered from scan_float, and returns a token
    /// containing everything matched by scan_number, scan_float, and this state.
    fn scan_float_exponent(&mut self, start: usize) -> Option<ScannerResult<'input>> {
        if let Some((_, c)) = self.current {
            if c == '-' {
                self.advance();
            }
        }
        loop {
            if let Some((i, c)) = self.current {
                if c.is_ascii_digit() {
                    self.advance();
                    continue;
                } else {
                    return self.good_token(
                        Token::FLOATLIT(self.input[start..i].to_string()),
                        start,
                        i,
                    );
                }
            } else {
                return self.good_token(
                    Token::FLOATLIT(self.input[start..].to_string()),
                    start,
                    self.input.len(),
                );
            }
        }
    }

    /// Scan a string literal.
    fn scan_string(&mut self, start: usize) -> Option<ScannerResult<'input>> {
        self.advance();
        loop {
            if let Some((i, c)) = self.current {
                match c {
                    '"' => {
                        self.advance();
                        return self.good_token(
                            Token::STRINGLIT(self.input[start + 1..i].to_string()),
                            start,
                            i + 1,
                        );
                    }
                    '\\' => {
                        self.advance();
                        match self.scan_string_escape() {
                            Err(e) => return Some(Err(e)),
                            Ok(_) => continue,
                        }
                    }
                    _ => self.advance(),
                }
            }
        }
    }

    fn scan_string_escape(&mut self) -> Result<char, CompilationError> {
        if let Some((pos, c)) = self.current {
            match c {
                '\\' => return Ok('\\'),
                'n' => return Ok('\n'),
                'r' => return Ok('\r'),
                '0' => return Ok('\0'),
                't' => return Ok('\t'),
                '"' => return Ok('"'),
                'x' => {
                    self.advance();
                    // scan two hex digits
                    let digits = self.swallow(2, 2, |q: char| q.is_ascii_hexdigit())?;
                    return Ok(char::from_u32(u32::from_str_radix(&digits, 16).unwrap()).unwrap());
                }
                'u' => {
                    self.advance();
                    self.swallow_char('{')?;
                    let digits = self.swallow(1, 6, |c| c.is_ascii_hexdigit())?;
                    self.swallow_char('}')?;
                    return Ok(char::from_u32(u32::from_str_radix(&digits, 16).unwrap()).unwrap());
                }
                c => {
                    let pos = self.line_and_col(pos);
                    return Err(CompilationError::InvalidEscape(pos, c.to_string()));
                }
            }
        } else {
            let pos = self.line_and_col(self.input.len());
            return Err(CompilationError::InvalidEscape(
                pos,
                "unterminated escape sequence".to_string(),
            ));
        }
    }

    /// Convenience function for scanning past a group of characters,
    /// adding them to the current token.
    ///
    /// Args:
    /// - min: the minimum number of characters to match.
    /// - max: the maximum number of characters to match.
    /// - pred: a function that returns true if a character is one
    ///    that should be matched.
    fn swallow(
        &mut self,
        min: usize,
        max: usize,
        pred: fn(c: char) -> bool,
    ) -> Result<String, CompilationError> {
        let mut result = String::new();
        for i in 0..max {
            if let Some((pos, c)) = self.current {
                if pred(c) {
                    result.push(c);
                    self.advance()
                } else {
                    if i >= min {
                        return Ok(result);
                    } else {
                        let pos = self.line_and_col(pos);
                        return Err(CompilationError::LexicalError(
                            pos,
                            format!("Invalid token: Expected at least {} chars", min).to_string(),
                        ));
                    }
                }
            } else {
                if i >= min {
                    return Ok(result);
                } else {
                    let pos = self.line_and_col(self.input.len());
                    return Err(CompilationError::LexicalError(
                        pos,
                        format!("Expected at least {} characters", min).to_string(),
                    ));
                }
            }
        }
        return Ok(result);
    }

    /// Similar to [swallow], but it only consumes a single, specific
    /// character.
    fn swallow_char(&mut self, c: char) -> Result<(), CompilationError> {
        if let Some((pos, q)) = self.current {
            if q == c {
                self.advance();
                return Ok(());
            } else {
                let loc = self.line_and_col(pos);
                return Err(CompilationError::LexicalError(
                    loc,
                    format!("Expected '{}', but saw '{}'", c, q).to_string(),
                ));
            }
        } else {
            let loc = self.line_and_col(self.input.len());
            return Err(CompilationError::LexicalError(
                loc,
                format!("Expected character, but saw EOF").to_string(),
            ));
        }
    }

    fn scan_char_escape(&mut self, start: usize) -> ScannerResult {
        let c = self.scan_string_escape()?;
        match self.current {
            Some((end, '\'')) => return self.good_token(Token::CHARLIT(c), start, end).unwrap(),
            _ => {
                let loc = self.line_and_col(start);
                return Err(CompilationError::LexicalError(
                    loc,
                    "Unterminated char literal".to_string(),
                ));
            }
        }
    }

    fn scan_char_literal(&mut self, start: usize) -> Option<ScannerResult> {
        self.advance();
        // After the "'", we should see either a single character,
        // or an escape code, followed by a single quote.
        if let Some((_, c)) = self.current {
            match c {
                '\\' => return Some(self.scan_char_escape(start)),
                _ => {
                    self.advance();
                    match self.current {
                        Some((end, '\'')) => {
                            self.advance();
                            return self.good_token(Token::CHARLIT(c), start, end);
                        }
                        Some((i, _)) => {
                            let loc = self.line_and_col(i);
                            return Some(Err(CompilationError::LexicalError(
                                loc,
                                "Invalid character literal".to_string(),
                            )));
                        }
                        _ => {
                            let loc = self.line_and_col(self.input.len());
                            return Some(Err(CompilationError::LexicalError(
                                loc,
                                "Invalid character literal".to_string(),
                            )));
                        }
                    }
                }
            }
        } else {
            let loc = self.line_and_col(start);
            return Some(Err(CompilationError::LexicalError(
                loc,
                "Invalid character literal".to_string(),
            )));
        }
    }

    fn scan_ident_or_keyword(&mut self, start: usize) -> Option<ScannerResult> {
        loop {
            match self.current {
                Some((_, c)) if c.is_id_char() => {
                    self.advance();
                    continue;
                }
                Some((pos, _)) => {
                    return self.ident_or_keyword(self.input[start..pos].to_string(), start, pos)
                }
                None => {
                    return self.ident_or_keyword(
                        self.input[start..].to_string(),
                        start,
                        self.input.len(),
                    )
                }
            }
        }
    }

    fn scan_at_ident(
        &mut self,
        start: usize,
    ) -> Option<Result<(Location, Token, Location), CompilationError>> {
        self.advance();
        loop {
            match self.current {
                Some((_, c)) if c.is_alphabetic() => {
                    self.advance();
                    continue;
                }
                Some((pos, _)) => {
                    return self.validate_at_keyword(&self.input[start..pos], start, pos)
                }
                None => {
                    return self.validate_at_keyword(&self.input[start..], start, self.input.len())
                }
            }
        }
    }

    fn scan_typevar(&mut self, start: usize) -> Option<ScannerResult> {
        self.advance();
        loop {
            match self.current {
                Some((_, c)) if c.is_alphabetic() => {
                    self.advance();
                    continue;
                }
                Some((end, _)) => {
                    return self.validate_typevar(&self.input[start..end], start, end)
                }
                None => {
                    return self.validate_typevar(&self.input[start..], start, self.input.len())
                }
            }
        }
    }

    fn scan_contextvar(&mut self, start: usize) -> Option<ScannerResult> {
        self.advance();
        loop {
            match self.current {
                Some((_, c)) if c.is_id_char() => {
                    self.advance();
                    continue;
                }
                Some((pos, _)) => {
                    return self.validate_contextvar(&self.input[start..], start, pos)
                }
                None => {
                    return self.validate_contextvar(&self.input[start..], start, self.input.len())
                }
            }
        }
    }

    fn scan_upper_ident(&mut self, start: usize) -> Option<ScannerResult> {
        loop {
            self.advance();
            match self.current {
                Some((_, c)) if c.is_id_char() => continue,
                Some((end, _)) => {
                    return self.good_token(
                        Token::UName(UpperName(self.input[start..end].to_string())),
                        start,
                        end,
                    )
                }
                None => {
                    return self.good_token(
                        Token::UName(UpperName(self.input[start..].to_string())),
                        start,
                        self.input.len(),
                    )
                }
            }
        }
    }

    fn skip_to_end_of_line(&mut self) {
        loop {
            self.advance();
            match self.current {
                Some((_, '\n')) => {
                    self.advance();
                    return;
                }
                Some(_) => (),
                None => return,
            }
        }
    }
}
