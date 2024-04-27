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

use std::fmt::{Display, Formatter};

pub type LowerName = String; // lowercase identifier - anything starting with a lowercase
                                  // letter, or a symbol other than ' or _

pub type UpperName = String; // uppercase identifier - anything starting with
                                  // an uppercase letter.

pub type TypeVarName = String; // type_var - anything starting with a '.

pub type ContextName = String; // context var - anything starting with a $.

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ModulePath(pub String);

impl ModulePath {
    pub fn new(segments: Vec<String>) -> ModulePath {
        ModulePath(segments.join("."))
    }

    pub fn to_file_syntax(&self) -> String {
        self.0.clone().replace('.', "/")
    }

    pub fn segments(&self) -> Vec<String> {
        self.0.clone().split(".").map(|s| s.to_string()).collect::<Vec<String>>()
    }
}

impl<'a> Display for ModulePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Ident<NameKind: Display> {
    Scoped {
        path: ModulePath,
        name: NameKind,
    },
    Local(NameKind),
}

impl<NameKind: Display> Display for Ident<NameKind> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scoped { path, name } => write!(
                f,
                "{}.{}",
                path.0,
                name
            ),
            Self::Local(n) => write!(f, "{}", n),
        }
    }
}

pub type TypeIdent = Ident<UpperName>;
pub type LowerIdent = Ident<LowerName>;
