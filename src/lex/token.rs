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
    Colon,
    Comma,
    DashDash,
    Arrow,
    Bang,
    Dot,

    LBracket,
    RBracket,
    LBracketBar,
    RBracketBar,
    LParen,
    RParen,
    LCurly,
    RCurly,

    Action,
    AtAction,
    Composes,
    Cond,
    AtCond,
    Do,
    End,
    Else,
    Exit,
    For,
    AtFor,
    Func,
    AtFunc,
    Init,
    Is,
    Local,
    Loop,
    AtLoop,
    Method,
    AtMethod,
    Next,
    New,
    Obj,
    AtObj,
    Sig,
    AtSig,
    Slot,
    AtSlot,
    Use,
    Var,
    AtVar,

    LName(LowerName),
    UName(UpperName),
    CName(ContextName),
    TVName(TypeVarName),
    IntLit(i64),
    FloatLit(String),
    StringLit(String),
    CharLit(char),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::DashDash => write!(f, "--"),
            Token::Arrow => write!(f, "->"),
            Token::Bang => write!(f, "!"),
            Token::Dot => write!(f, "."),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::LBracketBar => write!(f, "[|"),
            Token::RBracketBar => write!(f, "|]"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LCurly => write!(f, "{{"),
            Token::RCurly => write!(f, "}}"),
            Token::Action => write!(f, "action"),
            Token::AtAction => write!(f, "@action"),
            Token::Composes => write!(f, "composes"),
            Token::Cond => write!(f, "cond"),
            Token::AtCond => write!(f, "@cond"),
            Token::Do => write!(f, "do"),
            Token::End => write!(f, "end"),
            Token::Else => write!(f, "else"),
            Token::Exit => write!(f, "exit"),
            Token::For => write!(f, "for"),
            Token::AtFor => write!(f, "@for"),
            Token::Func => write!(f, "function"),
            Token::AtFunc => write!(f, "@function"),
            Token::Init => write!(f, "init"),
            Token::Is => write!(f, "is"),
            Token::Local => write!(f, "local"),
            Token::Loop => write!(f, "loop"),
            Token::AtLoop => write!(f, "@loop"),
            Token::Method => write!(f, "meth"),
            Token::AtMethod => write!(f, "@meth"),
            Token::Next => write!(f, "next"),
            Token::New => write!(f, "new"),
            Token::Obj => write!(f, "obj"),
            Token::AtObj => write!(f, "@obj"),
            Token::Sig => write!(f, "sig"),
            Token::AtSig => write!(f, "@sig"),
            Token::Slot => write!(f, "slot"),
            Token::AtSlot => write!(f, "@slot"),
            Token::Use => write!(f, "use"),
            Token::Var => write!(f, "var"),
            Token::AtVar => write!(f, "@var"),
            Token::LName(l) => write!(f, "LName({})", l),
            Token::UName(u) => write!(f, "UName({})", u),
            Token::CName(c) => write!(f, "CName({})", c),
            Token::TVName(t) => write!(f, "TVName({})", t),
            Token::IntLit(i) => write!(f, "Int({})", i),
            Token::FloatLit(fl) => write!(f, "Float({})", fl),
            Token::StringLit(s) => write!(f, "String({})", s),
            Token::CharLit(c) => write!(f, "Char({})", c),
        }
    }
}
