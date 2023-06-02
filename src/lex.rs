use crate::error::Error;
use line_col::LineColLookup;
use std::{collections::HashMap, str::CharIndices};
use unicode_categories::UnicodeCategories;

#[derive(Debug, PartialEq, Clone)]
pub enum Tok {
    SYMBOL(String),
    STACKVAR(String), //  @alpha+
    TYPEVAR(String),  //   'alpha+
    INTLIT(i64),
    FLOATLIT(f64),
    STRINGLIT(String),
    CHARLIT(char),
    // Keywords
    USE,
    STRUCT,
    END,
    SLOT,
    METH,
    FUN,
    IS,
    VAR,
    INIT,
    IF,
    ELSE,
    LOOP,

    // symbols
    BAR,     // |
    SUBTYPE, // <<
    SEND,    // <-

    LBRACE,   // {
    RBRACE,   //  }
    LBRACK,   // [
    RBRACK,   // ]
    POUND,    // #
    PLBRACK,  // #[
    PRBRACK,  // ]#
    PLBRACE,  // #{
    PRBRACE,  // }#
    BLOPEN,   // [[
    BLCLOSE,  // ]]
    LPAREN,   // (
    RPAREN,   // )
    DASHDASH, // --
    COLON,    // :
    COCO,     // ::
    COMMA,
}

/// An extension trait providing tests of a couple of
/// character categories that are useful for the parser.
trait CharacterCategories {
    fn is_id_start_char(&self) -> bool;
    fn is_id_char(&self) -> bool;
    fn is_syntax_char(&self) -> bool;
}

impl CharacterCategories for char {
    fn is_id_start_char(&self) -> bool {
        return !self.is_syntax_char()
            && !self.is_whitespace()
            && (self.is_alphabetic() || self.is_symbol() || self.is_punctuation());
    }

    fn is_id_char(&self) -> bool {
        return !self.is_syntax_char()
            && !self.is_whitespace()
            && (self.is_alphabetic()
                || self.is_punctuation()
                || self.is_symbol()
                || self.is_number());
    }

    fn is_syntax_char(&self) -> bool {
        match self {
            '\'' | '"' | '#' | '@' | '[' | ']' | '{' | '}' | '(' | ')' | ':' | ',' => true,
            _ => false,
        }
    }
}

pub struct Scanner<'input> {
    source_id: String,
    index: LineColLookup<'input>,
    chars: std::iter::Peekable<CharIndices<'input>>,
    input: &'input str,
    current: Option<(usize, char)>,
    next: Option<(usize, char)>,
    reserved: HashMap<String, Tok>,
}

impl<'input> Scanner<'input> {
    pub fn new(id: String, input: &'input str) -> Scanner<'input> {
        let mut scanner = Scanner {
            source_id: id,
            index: LineColLookup::new(input),
            chars: input.char_indices().peekable(),
            current: None,
            next: None,
            input,
            reserved: HashMap::from([
                ("use".to_string(), Tok::USE),
                ("struct".to_string(), Tok::STRUCT),
                ("end".to_string(), Tok::END),
                ("slot".to_string(), Tok::SLOT),
                ("meth".to_string(), Tok::METH),
                ("fun".to_string(), Tok::FUN),
                ("is".to_string(), Tok::IS),
                ("var".to_string(), Tok::VAR),
                ("init".to_string(), Tok::INIT),
                ("if".to_string(), Tok::IF),
                ("else".to_string(), Tok::ELSE),
                ("loop".to_string(), Tok::LOOP),
                ("|".to_string(), Tok::BAR),
                ("<<".to_string(), Tok::SUBTYPE),
                ("<-".to_string(), Tok::SEND),
                ("--".to_string(), Tok::DASHDASH),
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
    pub fn line_and_col(&self, pos: usize) -> (usize, usize) {
        self.index.get(pos)
    }

    fn advance(&mut self) {
        self.current = self.chars.next();
        self.next = match self.chars.peek() {
            Some(u) => Some(*u),
            None => None,
        }
    }
}

pub type ScannerResult<'input> = Result<(usize, Tok, usize), Error>;

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
    pub fn scan_token(&mut self) -> Option<ScannerResult<'input>> {
        loop {
            match self.current {
                // Skip WS
                Some((_, ' ')) | Some((_, '\n')) | Some((_, '\t')) => {
                    self.advance();
                    continue;
                }
                // the unambiguous single char tokens
                Some((idx, '{')) => {
                    self.advance();
                    return Some(Ok((idx, Tok::LBRACE, idx + 1)));
                }
                Some((idx, '(')) => {
                    self.advance();
                    return Some(Ok((idx, Tok::LPAREN, idx + 1)));
                }
                Some((idx, ')')) => {
                    self.advance();
                    return Some(Ok((idx, Tok::RPAREN, idx + 1)));
                }
                Some((idx, ',')) => {
                    self.advance();
                    return Some(Ok((idx, Tok::COMMA, idx + 1)));
                }
                Some((idx, '[')) => {
                    self.advance();
                    match self.current {
                        Some((_, '[')) => {
                            self.advance();
                            return Some(Ok((idx, Tok::BLOPEN, idx + 2)));
                        }
                        _ => return Some(Ok((idx, Tok::LBRACK, idx + 1))),
                    }
                }
                Some((idx, '#')) => {
                    self.advance();
                    match self.current {
                        Some((_, '[')) => {
                            self.advance();
                            return Some(Ok((idx, Tok::PLBRACK, idx + 2)));
                        }
                        Some((_, '{')) => {
                            self.advance();
                            return Some(Ok((idx, Tok::PLBRACE, idx + 2)));
                        }
                        _ => return Some(Ok((idx, Tok::POUND, idx + 1))),
                    }
                }
                Some((idx, ']')) => {
                    self.advance();
                    match self.current {
                        Some((_, '#')) => {
                            self.advance();
                            return Some(Ok((idx, Tok::PRBRACK, idx + 2)));
                        }
                        Some((_, ']')) => {
                            self.advance();
                            return Some(Ok((idx, Tok::BLCLOSE, idx + 2)));
                        }
                        _ => return Some(Ok((idx, Tok::RBRACK, idx + 1))),
                    }
                }
                Some((idx, '}')) => {
                    self.advance();
                    match self.current {
                        Some((_, '#')) => {
                            self.advance();
                            return Some(Ok((idx, Tok::PRBRACE, idx + 2)));
                        }
                        _ => return Some(Ok((idx, Tok::RBRACE, idx + 1))),
                    }
                }
                Some((idx, '/')) => {
                    self.advance();
                    match self.current {
                        Some((_, '*')) => match self.scan_past_comment(idx) {
                            Ok(_) => continue,
                            Err(e) => return Some(Err(e)),
                        },
                        Some((_, c)) if c.is_id_char() => return self.scan_id(idx),
                        _ => return Some(Ok((idx, Tok::SYMBOL("/".to_string()), idx + 1))),
                    }
                }

                Some((idx, ':')) => {
                    self.advance();
                    match self.current {
                        Some((_, ':')) => {
                            self.advance();
                            return Some(Ok((idx, Tok::COCO, idx + 2)));
                        }
                        _ => return Some(Ok((idx, Tok::COLON, idx + 1))),
                    }
                }
                Some((idx, '@')) => {
                    self.advance();
                    match self.current {
                        Some((_, c)) if c.is_alphabetic() => {
                            self.advance();
                            return Some(Ok((
                                idx,
                                Tok::STACKVAR("@".to_string() + &c.to_string()),
                                idx + 2,
                            )));
                        }
                        _ => {
                            let (line, column) = self.line_and_col(idx);
                            return Some(Err(Error::LexicalError {
                                line,
                                column,
                                message: "Invalid stack variable".to_string(),
                            }));
                        }
                    }
                }
                Some((idx, '\'')) => {
                    // char literal
                    return Some(self.scan_char_literal(idx));
                }
                Some((idx, '`')) => {
                    self.advance();
                    match self.current {
                        Some((_, c)) if c.is_alphabetic() => {
                            self.advance();
                            loop {
                                match self.current {
                                    Some((_, c)) if c.is_alphabetic() => self.advance(),
                                    Some((end, _)) => {
                                        return Some(Ok((
                                            idx,
                                            Tok::TYPEVAR(self.input[idx..end].to_string()),
                                            end - 1,
                                        )))
                                    }
                                    None => {
                                        return Some(Ok((
                                            idx,
                                            Tok::TYPEVAR(
                                                self.input[idx..self.input.len()].to_string(),
                                            ),
                                            self.input.len(),
                                        )))
                                    }
                                }
                            }
                        }
                        _ => {
                            let (line, column) = self.line_and_col(idx);
                            return Some(Err(Error::LexicalError {
                                line,
                                column,
                                message: "Invalid type variable".to_string(),
                            }));
                        }
                    }
                }
                Some((idx, '"')) => return self.scan_string(idx),
                Some((idx, c)) => {
                    if c == '-' {
                        // If it's a minus, and the next character is a digit,
                        // then send to number.
                        match self.next {
                            Some((_, c)) if c.is_number() => return self.scan_number(idx),
                            _ => (),
                        }
                    }
                    if c.is_id_start_char() {
                        return self.scan_id(idx);
                    } else if c.is_number() || c == '-' {
                        return self.scan_number(idx);
                    } else {
                        // error: skip past the error character, and then return the error.
                        self.advance();
                        let (line, column) = self.line_and_col(idx);
                        return Some(Err(Error::LexicalError {
                            line,
                            column,
                            message: format!("Invalid token char: {}", c),
                        }));
                    }
                }
                None => return None,
            }
        }
    }

    fn scan_id(&mut self, start: usize) -> Option<ScannerResult> {
        self.advance();
        loop {
            match self.current {
                Some((_, c)) if c.is_id_char() => self.advance(),
                Some((idx, _)) => {
                    return Some(Ok(self.id_or_reserved(
                        start,
                        idx,
                        self.input[start..idx].to_string(),
                    )))
                }
                None => {
                    return Some(Ok(self.id_or_reserved(
                        start,
                        self.input.len(),
                        self.input[start..self.input.len()].to_string(),
                    )))
                }
            }
        }
    }

    fn id_or_reserved(&self, start: usize, end: usize, name: String) -> (usize, Tok, usize) {
        if self.reserved.contains_key(&name) {
            return (start, self.reserved.get(&name).unwrap().clone(), end);
        } else {
            return (start, Tok::SYMBOL(name), end);
        }
    }

    fn scan_past_comment(&mut self, start: usize) -> Result<(), Error> {
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
                    let (line, column) = self.line_and_col(start);
                    return Err(Error::LexicalError {
                        line,
                        column,
                        message: "Unterminated comment".to_string(),
                    });
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
                    return Some(Ok((
                        start,
                        Tok::INTLIT(self.input[start..i].parse::<i64>().unwrap()),
                        i,
                    )));
                }
            } else {
                return Some(Ok((
                    start,
                    Tok::INTLIT(self.input[start..(start + count)].parse::<i64>().unwrap()),
                    start + count,
                )));
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
                    return Some(Ok((
                        start,
                        Tok::FLOATLIT(self.input[start..i].parse::<f64>().unwrap()),
                        i,
                    )));
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
                    return Some(Ok((
                        start,
                        Tok::FLOATLIT(self.input[start..i].parse::<f64>().unwrap()),
                        i,
                    )));
                }
            } else {
                return Some(Ok((
                    start,
                    Tok::FLOATLIT(self.input[start..].parse::<f64>().unwrap()),
                    self.input.len(),
                )));
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
                        return Some(Ok((
                            start,
                            Tok::STRINGLIT(self.input[start + 1..i].to_string()),
                            i + 1,
                        )));
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

    fn scan_string_escape(&mut self) -> Result<char, Error> {
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
                _ => {
                    let (line, column) = self.line_and_col(pos);
                    return Err(Error::LexicalError {
                        line,
                        column,
                        message: "Invalid escape sequence".to_string(),
                    });
                }
            }
        } else {
            let (line, column) = self.line_and_col(self.input.len());
            return Err(Error::LexicalError {
                line,
                column,
                message: "Unterminated escape sequence".to_string(),
            });
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
    ) -> Result<String, Error> {
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
                        let (line, column) = self.line_and_col(pos);
                        return Err(Error::LexicalError {
                            line,
                            column,
                            message: format!("Invalid token: Expected at least {} chars", min)
                                .to_string(),
                        });
                    }
                }
            } else {
                if i >= min {
                    return Ok(result);
                } else {
                    let (line, column) = self.line_and_col(self.input.len());
                    return Err(Error::LexicalError {
                        line,
                        column,
                        message: format!("Expected at least {} characters", min).to_string(),
                    });
                }
            }
        }
        return Ok(result);
    }

    /// Similar to [swallow], but it only consumes a single, specific
    /// character.
    fn swallow_char(&mut self, c: char) -> Result<(), Error> {
        if let Some((pos, q)) = self.current {
            if q == c {
                self.advance();
                return Ok(());
            } else {
                let (line, column) = self.line_and_col(pos);
                return Err(Error::LexicalError {
                    line,
                    column,
                    message: format!("Expected '{}', but saw '{}'", c, q).to_string(),
                });
            }
        } else {
            let (line, column) = self.line_and_col(self.input.len());
            return Err(Error::LexicalError {
                line,
                column,
                message: format!("Expected character, but saw EOF").to_string(),
            });
        }
    }

    fn scan_char_escape(&mut self, start: usize) -> ScannerResult {
        let c = self.scan_string_escape()?;
        match self.current {
            Some((end, '\'')) => return Ok((start, Tok::CHARLIT(c), end)),
            _ => {
                let (line, column) = self.line_and_col(start);
                return Err(Error::LexicalError {
                    line,
                    column,
                    message: "Unterminated char literal".to_string(),
                });
            }
        }
    }

    fn scan_char_literal(&mut self, start: usize) -> ScannerResult {
        self.advance();
        // After the "'", we should see either a single character,
        // or an escape code, followed by a single quote.
        if let Some((_, c)) = self.current {
            match c {
                '\\' => return self.scan_char_escape(start),
                _ => {
                    self.advance();
                    match self.current {
                        Some((end, '\'')) => {
                            self.advance();
                            return Ok((start, Tok::CHARLIT(c), end));
                        }
                        Some((i, _)) => {
                            let (line, column) = self.line_and_col(i);
                            return Err(Error::LexicalError {
                                line,
                                column,
                                message: "Invalid character literal".to_string(),
                            });
                        }
                        _ => {
                            let (line, column) = self.line_and_col(self.input.len());
                            return Err(Error::LexicalError {
                                line,
                                column,
                                message: "Invalid character literal".to_string(),
                            });
                        }
                    }
                }
            }
        } else {
            let (line, column) = self.line_and_col(start);
            return Err(Error::LexicalError {
                line,
                column,
                message: "Invalid character literal".to_string(),
            });
        }
    }
}
