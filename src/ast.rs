use crate::twist::{Twist, Twistable};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Symbol(pub String);

impl Symbol {
    fn to_string(&self) -> String {
        match self {
            Symbol(s) => *s,
        }
    }
}

pub enum Identifier {
    Qualified(Vec<Symbol>),
    Simple(Symbol),
    System(String),
}

impl Identifier {
    fn to_string(&self) -> String {
        match self {
            Self::Qualified(symbols) => symbols
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("::"),
            Self::Simple(s) => s.to_string(),
            Self::System(s) => format!("System({})", s).to_string(),
        }
    }
}

pub struct Sect {
    pub uses: Vec<UseDecl>,
    pub decls: Vec<Decl>,
}

impl Twistable for Sect {
    fn twist(&self) -> Twist {
        Twist::obj(
            "sect",
            vec![
                Twist::arr(
                    "uses",
                    self.uses.iter().map(|u| u.twist()).collect::<Vec<Twist>>(),
                ),
                Twist::arr(
                    "uses",
                    self.decls.iter().map(|s| s.twist()).collect::<Vec<Twist>>(),
                ),
            ],
        )
    }
}

pub struct UseDecl {
    pub sect: Identifier,
    pub names: Option<Vec<Symbol>>,
}

impl Twistable for UseDecl {
    fn twist(&self) -> Twist {
        match self.names {
            Some(names) => Twist::obj(
                "decl::use",
                vec![
                    Twist::attr("sect", self.sect.to_string()),
                    Twist::arr(
                        "names",
                        names
                            .iter()
                            .map(|n| Twist::attr("name", n.to_string()))
                            .collect::<Vec<Twist>>(),
                    ),
                ],
            ),
            None => Twist::obj(
                "decl::use",
                vec![Twist::attr("sect", self.sect.to_string())],
            ),
        }
    }
}

pub enum Decl {
    Struct(StructDecl),
    Function(FunctionDecl),
    Var(VarDecl),
}

impl Twistable for Decl {
    fn twist(&self) -> Twist {
        match self {
            Self::Struct(s) => s.twist(),
            Self::Function(f) => f.twist(),
            Self::Var(v) => v.twist(),
        }
    }
}

pub struct TypeParam {
    pub name: Symbol,
    pub constraint: Option<SType>,
}

impl Twistable for TypeParam {
    fn twist(&self) -> Twist {
        match self.constraint {
            Some(c) => Twist::obj(
                "TypeParam",
                vec![
                    Twist::attr("name", self.name.to_string()),
                    Twist::val("constraint", c.twist()),
                ],
            ),
            None => Twist::attr("TypeParam", self.name),
        }
    }
}

pub struct StructDecl {
    pub name: Symbol,
    pub supers: Option<Vec<SType>>,
    pub type_params: Option<Vec<TypeParam>>,
    pub fields: Vec<TypedIdentifier>,
    pub methods: Vec<MethodDecl>,
}

fn twist_vec<T: Twistable>(vs: Vec<T>) -> Vec<Twist> {
    vs.iter().map(|v| v.twist()).collect::<Vec<Twist>>()
}

impl Twistable for StructDecl {
    fn twist(&self) -> Twist {
        let mut children: Vec<Twist> = Vec::new();
        children.push(Twist::attr("name", self.name.to_string()));
        match self.supers {
            Some(sups) => children.push(Twist::arr("supers", twist_vec(sups))),
            None => (),
        }
        match self.type_params {
            Some(tps) => children.push(Twist::arr("type_params", twist_vec(tps))),
            None => (),
        }
        children.push(Twist::arr("fields", twist_vec(self.fields)));
        children.push(Twist::arr("methods", twist_vec(self.methods)));
        return Twist::obj("decl::struct", children);
    }
}

pub enum StructMemberDecl {
    Field(TypedIdentifier),
    Method(MethodDecl),
}

pub struct MethodDecl {
    pub name: Symbol,
    pub effect: StackEffect,
    pub body: Vec<Expr>,
}

impl Twistable for MethodDecl {
    fn twist(&self) -> Twist {
        Twist::obj(
            "decl::method",
            vec![
                Twist::attr("name", self.name.to_string()),
                Twist::val("effect", self.effect.twist()),
                Twist::arr("body", twist_vec(self.body)),
            ],
        )
    }
}

pub struct TypedIdentifier {
    pub name: Symbol,
    pub s_type: SType,
}

impl Twistable for TypedIdentifier {
    fn twist(&self) -> Twist {
        Twist::obj(
            "TypedId",
            vec![
                Twist::attr("name", self.name.to_string()),
                Twist::val("type", self.s_type.twist()),
            ],
        )
    }
}

pub struct FunctionDecl {
    pub name: Symbol,
    pub type_params: Option<Vec<TypeParam>>,
    pub signature: StackEffect,
    pub body: Vec<Expr>,
}

impl Twistable for FunctionDecl {
    fn twist(&self) -> Twist {
        let mut children: Vec<Twist> = Vec::new();
        children.push(Twist::attr("name", self.name.to_string()));
        match self.type_params {
            Some(tps) => children.push(Twist::arr("TypeParams", twist_vec(tps))),
            None => (),
        }
        children.push(Twist::val("effect", self.signature.twist()));
        children.push(Twist::arr("body", twist_vec(self.body)));
        return Twist::obj("decl::function", children);
    }
}

pub struct VarDecl {
    pub name: Symbol,
    pub s_type: SType,
    pub init_value: Vec<Expr>,
}

impl Twistable for VarDecl {
    fn twist(&self) -> Twist {
        Twist::obj(
            "decl::var",
            vec![
                Twist::attr("name", self.name.to_string()),
                Twist::val("type", self.s_type.twist()),
                Twist::arr("init_value", twist_vec(self.init_value)),
            ],
        )
    }
}

pub enum Expr {
    FunCall(FunCallExpr),
    List(ListExpr),
    Map(MapExpr),
    Cond(CondExpr),
    Loop(LoopExpr),
    MethodCall(MethodCallExpr),
    Block(BlockExpr),
    IntLit(String),
    FloatLit(String),
    StringLit(String),
    CharLit(String),
}

impl Twistable for Expr {
    fn twist(&self) -> Twist {
        match self {
            Self::FunCall(f) => f.twist(),
            Self::List(l) => l.twist(),
            Self::Map(m) => m.twist(),
            Self::Cond(c) => c.twist(),
            Self::Loop(l) => l.twist(),
            Self::MethodCall(m) => m.twist(),
            Self::Block(b) => b.twist(),
            Self::IntLit(i) => Twist::attr("IntLit", i.to_string()),
            Self::FloatLit(f) => Twist::attr("FloatLit", f.to_string()),
            Self::StringLit(s) => Twist::attr("StringLit", s.to_string()),
            Self::CharLit(c) => Twist::attr("CharLit", c.to_string()),
        }
    }
}

pub enum SType {
    Simple(Identifier),
    Parametric(Vec<SType>, Identifier),
    Function(StackEffect),
    TypeVar(Symbol),
}

impl Twistable for SType {
    fn twist(&self) -> Twist {
        match self {
            Self::Simple(id) => Twist::attr("type::simple", id.to_string()),
            Self::Parametric(params, name) => Twist::obj(
                "type::param",
                vec![
                    Twist::attr("id", name.to_string()),
                    Twist::arr("type_params", twist_vec(*params)),
                ],
            ),
            Self::Function(eff) => Twist::val("type::func", eff.twist()),
            Self::TypeVar(s) => Twist::attr("type:var", s.to_string()),
        }
    }
}

pub struct StackEffect {
    pub before: StackImage,
    pub after: StackImage,
}

impl Twistable for StackEffect {
    fn twist(&self) -> Twist {
        Twist::obj(
            "StackEffect",
            vec![
                Twist::val("before", self.before.twist()),
                Twist::val("after", self.after.twist()),
            ],
        )
    }
}

pub struct StackImage {
    pub stack_var: Symbol,
    pub stack: Vec<SType>,
}

impl Twistable for StackImage {
    fn twist(&self) -> Twist {
        Twist::obj(
            "StackImage",
            vec![
                Twist::attr("stackvar", self.stack_var.to_string()),
                Twist::arr("stack", twist_vec(self.stack)),
            ],
        )
    }
}

static IMG_VARIABLE_INDEX: AtomicUsize = AtomicUsize::new(0);

impl StackImage {
    pub fn unique_image_var() -> String {
        let idx = IMG_VARIABLE_INDEX.fetch_add(1, Ordering::Relaxed);
        return format!("__stack__{}__", idx);
    }
}

// A function call is just the name of the function.
pub struct FunCallExpr(pub Identifier);

pub struct ListExpr {
    pub value_type: SType,
    pub values: Vec<Vec<Expr>>,
}

pub struct MapExpr {
    pub key_type: SType,
    pub value_type: SType,
    pub values: Vec<(Vec<Expr>, Vec<Expr>)>,
}

pub struct CondExpr {
    pub true_block: Vec<Expr>,
    pub false_block: Vec<Expr>,
}

pub struct LoopExpr(pub Vec<Expr>);

pub struct MethodCallExpr(pub Symbol);

pub struct BlockExpr(pub StackEffect, pub Vec<Expr>);
