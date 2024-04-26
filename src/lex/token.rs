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

use std::fmt::Display;

use super::{ContextName, LowerName, TypeVarName, UpperName};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    COLON,
    COMMA,
    DASHDASH,
    ARROW,
    BANG,
    DOT,

    LBRACK,
    RBRACK,
    LBRACKBAR,
    RBRACKBAR,
    LPAREN,
    RPAREN,
    LCURLY,
    RCURLY,

    ACTION,
    ATACTION,
    COMPOSES,
    COND,
    ATCOND,
    DO,
    END,
    ELSE,
    EXIT,
    FOR,
    ATFOR,
    FUNCTION,
    ATFUNCTION,
    INIT,
    IS,
    LOCAL,
    LOOP,
    ATLOOP,
    METHOD,
    ATMETHOD,
    NEXT,
    NEW,
    OBJ,
    ATOBJ,
    SIG,
    ATSIG,
    SLOT,
    ATSLOT,
    USE,
    VAR,
    ATVAR,

    LName(LowerName),
    UName(UpperName),
    CName(ContextName),
    TVName(TypeVarName),
    INTLIT(i64),
    FLOATLIT(String),
    STRINGLIT(String),
    CHARLIT(char),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::COLON => write!(f, ":"),
            Token::COMMA => write!(f, ","),
            Token::DASHDASH => write!(f, "--"),
            Token::ARROW => write!(f, "->"),
            Token::BANG => write!(f, "!"),
            Token::DOT => write!(f, "."),
            Token::LBRACK => write!(f, "["),
            Token::RBRACK => write!(f, "]"),
            Token::LBRACKBAR => write!(f, "[|"),
            Token::RBRACKBAR => write!(f, "|]"),
            Token::LPAREN => write!(f, "("),
            Token::RPAREN => write!(f, ")"),
            Token::LCURLY => write!(f, "{{"),
            Token::RCURLY => write!(f, "}}"),
            Token::ACTION => write!(f, "action"),
            Token::ATACTION => write!(f, "@action"),
            Token::COMPOSES => write!(f, "composes"),
            Token::COND => write!(f, "cond"),
            Token::ATCOND => write!(f, "@cond"),
            Token::DO => write!(f, "do"),
            Token::END => write!(f, "end"),
            Token::ELSE => write!(f, "else"),
            Token::EXIT => write!(f, "exit"),
            Token::FOR => write!(f, "for"),
            Token::ATFOR => write!(f, "atfor"),
            Token::FUNCTION => write!(f, "function"),
            Token::ATFUNCTION => write!(f, "@function"),
            Token::INIT => write!(f, "init"),
            Token::IS => write!(f, "is"),
            Token::LOCAL => write!(f, "local"),
            Token::LOOP => write!(f, "loop"),
            Token::ATLOOP => write!(f, "@loop"),
            Token::METHOD => write!(f, "meth"),
            Token::ATMETHOD => write!(f, "@meth"),
            Token::NEXT => write!(f, "next"),
            Token::NEW => write!(f, "new"),
            Token::OBJ => write!(f, "obj"),
            Token::ATOBJ => write!(f, "@obj"),
            Token::SIG => write!(f, "sig"),
            Token::ATSIG => write!(f, "@sig"),
            Token::SLOT => write!(f, "slot"),
            Token::ATSLOT => write!(f, "@slot"),
            Token::USE => write!(f, "use"),
            Token::VAR => write!(f, "var"),
            Token::ATVAR => write!(f, "@var"),
            Token::LName(l) => write!(f, "LName({})", l),
            Token::UName(u) => write!(f, "UName({})", u),
            Token::CName(c) => write!(f, "CName({})", c),
            Token::TVName(t) => write!(f, "TVName({})", t),
            Token::INTLIT(i) => write!(f, "Int({})", i),
            Token::FLOATLIT(fl) => write!(f, "Float({})", fl),
            Token::STRINGLIT(s) => write!(f, "String({})", s),
            Token::CHARLIT(c) => write!(f, "Char({})", c),
        }
    }
}
