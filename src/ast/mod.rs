use std::fmt::Display;

use crate::twist::{Twist, Twistable};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LowerName(pub String); // lowercase identifier - anything starting with a lowercase
                                  // letter, or a symbol other than ' or _

impl Display for LowerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UpperName(pub String); // uppercase identifier - anything starting with
                                  // an uppercase letter.

impl Display for UpperName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypeVarName(pub String); // typevar - anything starting with a '.

impl Display for TypeVarName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ContextName(pub String); // context var - anything starting with a $.

impl Display for ContextName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type ModulePath = Vec<LowerName>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Ident<NameKind: Display> {
    Scoped { path: ModulePath, name: NameKind },
    Local(NameKind),
}

impl<NK: Display> Display for Ident<NK> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scoped { path, name } => write!(
                f,
                "{}.{}",
                path.iter()
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

#[derive(Debug, PartialEq, Clone)]
pub struct Sect {
    pub uses: Vec<UseDecl>,
    pub defs: Vec<Definition>,
}

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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UseDecl {
    pub sect: ModulePath,
    pub type_names: Vec<UpperName>,
    pub function_names: Vec<LowerName>,
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SchismType {
    ParametricType(TypeIdent, Vec<Box<SchismType>>),
    SimpleType(TypeIdent),
    UnboundTypeVar(TypeVarName),
    FunctionType(StackEffect),
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum StackEntry {
    TypeEntry(Box<SchismType>),
    NamedEntry(LowerName, Box<SchismType>),
}

impl Twistable for StackEntry {
    fn twist(&self) -> Twist {
        match self {
            Self::TypeEntry(st) => st.twist(),
            Self::NamedEntry(name, st) => Twist::val(&name.to_string(), st.twist()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StackImage {
    pub context: Option<ContextName>,
    pub stack: Vec<StackEntry>,
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StackEffect {
    pub effect_domains: Vec<TypeIdent>,
    pub before: StackImage,
    pub after: StackImage,
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Definition {
    Sig(SigDef),
    Obj(ObjectDef),
    Function(FunctionDef),
    Var(VarDef),
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SigDef {
    pub name: UpperName,
    pub type_params: Vec<TypeParam>,
    pub composes: Vec<SchismType>,
    pub operations: Vec<OperationSig>,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypeParam {
    pub name: TypeVarName,
    pub constraint: Option<SchismType>,
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OperationSig {
    MethodSig {
        name: LowerName,
        effect: StackEffect,
    },
    ActionSig {
        name: LowerName,
        inputs: StackImage,
    },
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ObjectDef {
    pub name: UpperName,
    pub type_params: Vec<TypeParam>,
    pub composes: Vec<SchismType>,
    pub inputs: StackImage,
    pub members: Vec<ObjectMemberDecl>,
    pub body: Vec<Statement>,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ObjectMemberDecl {
    ObjectSlot {
        name: LowerName,
        value_type: SchismType,
        inputs: StackImage,
        body: Vec<Statement>,
    },
    ObjectMethod {
        name: LowerName,
        effect: StackEffect,
        body: Vec<Statement>,
    },
    ObjectAction {
        name: LowerName,
        inputs: StackImage,
        body: Vec<Statement>,
    },
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionDef {
    pub name: LowerName,
    pub type_params: Vec<TypeParam>,
    pub effect: StackEffect,
    pub body: Vec<Statement>,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VarDef {
    pub name: LowerName,
    pub value_type: SchismType,
    pub inputs: StackImage,
    pub body: Vec<Statement>,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement {
    StringLit(String),
    IntLit(i64),
    FloatLit(String),
    CharLit(char),
    Sequence(Vec<Box<Statement>>),
    Name(Ident<LowerName>),
    Block(Vec<Box<Statement>>, StackEffect),
    Local(LowerName),
    New(SchismType),
    DispatchStmt(Ident<LowerName>),
    CallStmt(Ident<LowerName>),
    CondStmt {
        clauses: Vec<CondClause>,
        else_clause: Vec<Box<Statement>>,
    },
    LoopStmt {
        label: Option<LowerName>,
        body: Vec<Box<Statement>>,
    },
    ForStmt {
        idx: LowerName,
        body: Vec<Box<Statement>>,
    },
    ExitStmt(Option<LowerName>),
    NextStmt(Option<LowerName>),
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CondClause {
    pub condition: Box<Statement>,
    pub body: Vec<Box<Statement>>,
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
