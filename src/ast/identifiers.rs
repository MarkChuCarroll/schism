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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LowerName(pub String); // lowercase identifier - anything starting with a lowercase
                                  // letter, or a symbol other than ' or _

impl Display for LowerName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UpperName(pub String); // uppercase identifier - anything starting with
                                  // an uppercase letter.

impl Display for UpperName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TypeVarName(pub String); // type_var - anything starting with a '.

impl Display for TypeVarName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ContextName(pub String); // context var - anything starting with a $.

impl Display for ContextName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ModulePath {
    pub segments: Vec<LowerName>
}

impl Display for ModulePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for seg in &self.segments {
            write!(f, ".{}", seg.to_string())?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Ident<NameKind: Display> {
    Scoped { path: ModulePath, name: NameKind },
    Local(NameKind),
}

impl<NameKind: Display> Display for Ident<NameKind> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scoped { path, name } => write!(
                f,
                "{}.{}",
                path.segments.iter()
                    .map(|it| it.to_string())
                    .collect::<Vec<String>>()
                    .join("::"),
                name
            ),
            Self::Local(n) => write!(f, "{}", n),
        }
    }
}

pub type TypeIdent = Ident<UpperName>;
pub type LowerIdent = Ident<LowerName>;
