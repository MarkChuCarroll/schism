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

use super::*;
use crate::twist::{Twist, Twistable};

impl Twistable for Sect {
    fn twist(&self) -> Twist {
        Twist::obj(
            "sect",
            vec![
                Twist::twist_arr("uses", &self.uses),
                Twist::twist_arr("defs", &self.defs),
            ],
        )
    }
}

impl Twistable for UseDecl {
    fn twist(&self) -> Twist {
        Twist::obj(
            "UseDecl",
            vec![
                Twist::arr(
                    "types",
                    self.type_names
                        .iter()
                        .map(|it| Twist::leaf(&it.to_string()))
                        .collect::<Vec<Twist>>(),
                ),
                Twist::arr(
                    "functions",
                    self.function_names
                        .iter()
                        .map(|it| Twist::leaf(&it.to_string()))
                        .collect::<Vec<Twist>>(),
                ),
            ],
        )
    }
}

impl Twistable for SchismType {
    fn twist(&self) -> Twist {
        match self {
            Self::ParametricType(id, args) => Twist::obj(
                "ParametricType",
                vec![
                    Twist::attr("base_type", id.to_string()),
                    Twist::twist_arr("parameters", args),
                ],
            ),
            Self::SimpleType(id) => {
                Twist::obj("SimpleType", vec![Twist::attr("id", id.to_string())])
            }
            Self::UnboundTypeVar(id) => {
                Twist::obj("TypeVar", vec![Twist::attr("id", id.to_string())])
            }
            Self::FunctionType(effect) => Twist::val("FunctionType", effect.twist()),
        }
    }
}

impl Twistable for StackEntry {
    fn twist(&self) -> Twist {
        match self {
            Self::TypeEntry(st) => st.twist(),
            Self::NamedEntry(name, st) => Twist::val(&name.to_string(), st.twist()),
        }
    }
}

impl Twistable for StackImage {
    fn twist(&self) -> Twist {
        Twist::obj(
            "StackImage",
            vec![
                Twist::twist_opt_val(
                    "context",
                    self.context.as_ref().map(|it| Twist::leaf(&it.to_string())),
                ),
                Twist::twist_arr("Stack", &self.stack),
            ],
        )
    }
}

impl Twistable for TypeIdent {
    fn twist(&self) -> Twist {
        Twist::leaf(&self.to_string())
    }
}

impl Twistable for StackEffect {
    fn twist(&self) -> Twist {
        Twist::obj(
            "StackEffect",
            vec![
                Twist::twist_arr("effect_domains", &self.effect_domains),
                Twist::twist_val("before", &self.before),
                Twist::twist_val("after", &self.after),
            ],
        )
    }
}

impl Twistable for Definition {
    fn twist(&self) -> Twist {
        match self {
            Self::Sig(s) => s.twist(),
            Self::Obj(o) => o.twist(),
            Self::Function(f) => f.twist(),
            Self::Var(v) => v.twist(),
        }
    }
}

impl Twistable for SigDef {
    fn twist(&self) -> Twist {
        Twist::obj(
            "SigDef",
            vec![
                Twist::attr("name", self.name.to_string()),
                Twist::twist_arr("type_params", &self.type_params),
                Twist::twist_arr("composes", &self.composes),
                Twist::twist_arr("operations", &self.operations),
            ],
        )
    }
}

impl Twistable for TypeParam {
    fn twist(&self) -> Twist {
        Twist::obj(
            "TypeParam",
            vec![
                Twist::attr("name", self.name.to_string()),
                Twist::opt_val("constraint", self.constraint.as_ref().map(|it| it.twist())),
            ],
        )
    }
}

impl Twistable for OperationSig {
    fn twist(&self) -> Twist {
        match self {
            Self::MethodSig { name, effect } => Twist::obj(
                "MethodSig",
                vec![
                    Twist::attr("name", name.to_string()),
                    Twist::twist_val("effect", effect),
                ],
            ),
            Self::ActionSig { name, inputs } => Twist::obj(
                "ActionSig",
                vec![
                    Twist::attr("name", name.to_string()),
                    Twist::twist_val("inputs", inputs),
                ],
            ),
        }
    }
}

impl Twistable for ObjectDef {
    fn twist(&self) -> Twist {
        Twist::obj(
            "ObjectDef",
            vec![
                Twist::attr("name", self.name.to_string()),
                Twist::twist_arr("type_params", &self.type_params),
                Twist::twist_arr("composes", &self.composes),
                Twist::twist_val("inputs", &self.inputs),
                Twist::twist_arr("members", &self.members),
                Twist::twist_arr("body", &self.body),
            ],
        )
    }
}

impl Twistable for ObjectMemberDecl {
    fn twist(&self) -> Twist {
        match self {
            ObjectMemberDecl::ObjectSlot {
                name,
                value_type,
                inputs,
                body,
            } => Twist::obj(
                "Slot",
                vec![
                    Twist::attr("name", name.to_string()),
                    Twist::twist_val("value_type", value_type),
                    Twist::twist_val("inputs", inputs),
                    Twist::twist_arr("body", body),
                ],
            ),
            ObjectMemberDecl::ObjectMethod { name, effect, body } => Twist::obj(
                "Method",
                vec![
                    Twist::attr("name", name.to_string()),
                    Twist::twist_val("effect", effect),
                    Twist::twist_arr("body", body),
                ],
            ),
            ObjectMemberDecl::ObjectAction { name, inputs, body } => Twist::obj(
                "Action",
                vec![
                    Twist::attr("name", name.to_string()),
                    Twist::twist_val("inputs", inputs),
                    Twist::twist_arr("body", body),
                ],
            ),
        }
    }
}

impl Twistable for FunctionDef {
    fn twist(&self) -> Twist {
        Twist::obj(
            "Function",
            vec![
                Twist::attr("name", self.name.to_string()),
                Twist::twist_arr("type_params", &self.type_params),
                Twist::twist_val("effect", &self.effect),
                Twist::twist_arr("body", &self.body),
            ],
        )
    }
}

impl Twistable for VarDef {
    fn twist(&self) -> Twist {
        Twist::obj(
            "Variable",
            vec![
                Twist::attr("name", self.name.to_string()),
                Twist::twist_val("value_type", &self.value_type),
                Twist::twist_val("inputs", &self.inputs),
                Twist::twist_arr("body", &self.body),
            ],
        )
    }
}

impl Twistable for Statement {
    fn twist(&self) -> Twist {
        match self {
            Statement::StringLit(s) => Twist::attr("StringLit", s.to_string()),
            Statement::IntLit(i) => Twist::attr("IntLit", i.to_string()),
            Statement::FloatLit(f) => Twist::attr("FloatLit", f.to_string()),
            Statement::CharLit(c) => Twist::attr("CharLit", format!("'{}'", c)),
            Statement::Sequence(s) => Twist::twist_arr("Sequence", s),
            Statement::Name(n) => Twist::attr("InvokeName", n.to_string()),
            Statement::Block(body, effect) => Twist::obj(
                "Block",
                vec![
                    Twist::twist_val("effect", effect),
                    Twist::twist_arr("body", body),
                ],
            ),
            Statement::Local(id) => Twist::attr("Local", id.to_string()),
            Statement::New(id) => Twist::twist_val("New", id),
            Statement::DispatchStmt(name) => Twist::attr("Dispatch", name.to_string()),
            Statement::CallStmt(c) => Twist::attr("MethodCall", c.to_string()),
            Statement::CondStmt {
                clauses,
                else_clause,
            } => Twist::obj(
                "Cond",
                vec![
                    Twist::twist_arr("cases", clauses),
                    Twist::twist_arr("else", else_clause),
                ],
            ),
            Statement::LoopStmt { label, body } => Twist::obj(
                "Loop",
                vec![
                    Twist::opt_val(
                        "label",
                        label.as_ref().map(|it| Twist::leaf(&it.to_string())),
                    ),
                    Twist::twist_arr("body", body),
                ],
            ),
            Statement::ForStmt { idx, body } => Twist::obj(
                "For",
                vec![
                    Twist::attr("index", idx.to_string()),
                    Twist::twist_arr("body", body),
                ],
            ),
            Statement::ExitStmt(l) => Twist::obj(
                "Exit",
                vec![Twist::opt_val(
                    "loop_label",
                    l.as_ref().map(|it| Twist::leaf(&it.to_string())),
                )],
            ),
            Statement::NextStmt(l) => Twist::obj(
                "Next",
                vec![Twist::twist_opt_val(
                    "loop_label",
                    l.as_ref().map(|it| Twist::leaf(&it.to_string())),
                )],
            ),
        }
    }
}

impl Twistable for CondClause {
    fn twist(&self) -> Twist {
        Twist::obj(
            "CondClause",
            vec![
                Twist::twist_val("condition", &self.condition),
                Twist::twist_arr("body", &self.body),
            ],
        )
    }
}

impl<T: Twistable> Twistable for Box<T> {
    fn twist(&self) -> Twist {
        let t = self.as_ref();
        t.twist()
    }
}
