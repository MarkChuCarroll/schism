use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Symbol(pub String);

pub trait Renderable {
    fn render_into(&self, target: &mut String, indent: usize);

    fn to_string(&self) -> String {
        let mut s = String::new();
        self.render_into(&mut s, 1);
        return s;
    }

    fn indent(&self, target: &mut String, ind: usize) {
        target.push_str(&"   ".repeat(ind))
    }
}

impl Symbol {
    fn to_string(&self) -> String {
        match self {
            Symbol(s) => s.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct Sect {
    pub uses: Vec<UseDecl>,
    pub decls: Vec<Decl>,
}

impl Renderable for Sect {
    fn render_into(&self, target: &mut String, indent: usize) {
        self.indent(target, indent);
        target.push_str("sect\n");
        for u in &self.uses {
            u.render_into(target, indent + 1);
        }
        for d in &self.decls {
            d.render_into(target, indent + 1)
        }
        self.indent(target, indent);
        target.push_str("end\n");
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UseDecl {
    pub sect: Identifier,
    pub names: Option<Vec<Symbol>>,
}

impl Renderable for UseDecl {
    fn render_into(&self, target: &mut String, indent: usize) {
        self.indent(target, indent);
        target.push_str("use ");
        target.push_str(&self.sect.to_string());
        match &self.names {
            Some(vs) => {
                target.push_str("{");
                target.push_str(
                    &vs.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                target.push_str("}\n");
            }
            None => target.push_str("\n"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Decl {
    Struct(StructDecl),
    Function(FunctionDecl),
    Var(VarDecl),
}

impl Renderable for Decl {
    fn render_into(&self, target: &mut String, indent: usize) {
        match self {
            Self::Struct(s) => s.render_into(target, indent),
            Self::Function(f) => f.render_into(target, indent),
            Self::Var(v) => v.render_into(target, indent),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypeParam {
    pub name: Symbol,
    pub constraint: Option<SType>,
}

impl Renderable for TypeParam {
    fn render_into(&self, target: &mut String, indent: usize) {
        target.push_str(&self.name.to_string());
        match &self.constraint {
            Some(st) => {
                target.push_str("<<");
                st.render_into(target, indent);
            }
            None => (),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructDecl {
    pub name: Symbol,
    pub supers: Option<Vec<SType>>,
    pub type_params: Option<Vec<TypeParam>>,
    pub fields: Vec<TypedIdentifier>,
    pub methods: Vec<MethodDecl>,
}

impl Renderable for StructDecl {
    fn render_into(&self, target: &mut String, indent: usize) {
        self.indent(target, indent);
        target.push_str("struct ");
        match &self.type_params {
            Some(tps) => {
                target.push_str("[");
                target.push_str(
                    &tps.iter()
                        .map(|tp| tp.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                target.push_str("]");
            }
            None => (),
        }
        target.push_str(&self.name.to_string());

        target.push_str("\n");
        match &self.supers {
            Some(ss) => {
                self.indent(target, indent + 1);
                target.push_str("supers ");
                target.push_str(
                    &ss.iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                target.push_str("\n");
            }
            None => (),
        }
        for f in &self.fields {
            self.indent(target, indent + 1);
            target.push_str("slot ");
            target.push_str(&f.name.to_string());
            target.push_str(": ");
            target.push_str(&f.s_type.to_string());
            target.push_str("\n")
        }
        for m in &self.methods {
            m.render_into(target, indent + 1)
        }
        self.indent(target, indent);
        target.push_str("end\n")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StructMemberDecl {
    Field(TypedIdentifier),
    Method(MethodDecl),
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodDecl {
    pub name: Symbol,
    pub effect: StackEffect,
    pub body: Vec<Expr>,
}

impl Renderable for MethodDecl {
    fn render_into(&self, target: &mut String, indent: usize) {
        self.indent(target, indent);
        target.push_str("meth ");
        target.push_str(&self.name.to_string());
        target.push_str(" ");
        target.push_str(&self.effect.to_string());
        target.push_str(" do\n");
        for b in &self.body {
            b.render_into(target, indent + 1)
        }
        self.indent(target, indent);
        target.push_str("end\n")
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypedIdentifier {
    pub name: Symbol,
    pub s_type: SType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDecl {
    pub name: Symbol,
    pub type_params: Option<Vec<TypeParam>>,
    pub signature: StackEffect,
    pub body: Vec<Expr>,
}

impl Renderable for FunctionDecl {
    fn render_into(&self, target: &mut String, indent: usize) {
        self.indent(target, indent);
        target.push_str("fun ");
        match &self.type_params {
            Some(tps) => {
                target.push_str("[");
                target.push_str(
                    &tps.iter()
                        .map(|tp| tp.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                target.push_str("]");
            }
            None => (),
        }
        target.push_str(&self.name.to_string());
        target.push_str(&self.signature.to_string());
        target.push_str(" is\n");
        for b in &self.body {
            b.render_into(target, indent + 1);
        }
        self.indent(target, indent);
        target.push_str("end\n")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarDecl {
    pub name: Symbol,
    pub s_type: SType,
    pub init_value: Vec<Expr>,
}

impl Renderable for VarDecl {
    fn render_into(&self, target: &mut String, indent: usize) {
        self.indent(target, indent);
        target.push_str("var ");
        target.push_str(&self.name.to_string());
        target.push_str(": ");
        target.push_str(&self.s_type.to_string());
        target.push_str("{\n");
        for e in &self.init_value {
            e.render_into(target, indent + 1)
        }
        self.indent(target, indent);
        target.push_str("}\n")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    FunCall(FunCallExpr),
    List(ListExpr),
    Map(MapExpr),
    Cond(CondExpr),
    Loop(LoopExpr),
    MethodCall(MethodCallExpr),
    Block(BlockExpr),
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
    CharLit(char),
    Local(LocalExpr),
}

impl Renderable for Expr {
    fn render_into(&self, target: &mut String, indent: usize) {
        match self {
            Self::FunCall(f) => f.render_into(target, indent),
            Self::List(l) => l.render_into(target, indent),
            Self::Map(m) => m.render_into(target, indent),
            Self::Cond(c) => c.render_into(target, indent),
            Self::Loop(l) => l.render_into(target, indent),
            Self::MethodCall(m) => m.render_into(target, indent),
            Self::Block(b) => b.render_into(target, indent),
            Self::IntLit(i) => {
                self.indent(target, indent);
                target.push_str(&i.to_string());
                target.push_str("\n")
            }
            Self::FloatLit(f) => {
                self.indent(target, indent);
                target.push_str(&f.to_string());
                target.push_str("\n")
            }
            Self::StringLit(s) => {
                self.indent(target, indent);
                target.push('"');
                target.push_str(s);
                target.push_str("\"\n")
            }
            Self::CharLit(c) => {
                self.indent(target, indent);
                target.push_str("'");
                target.push(*c);
                target.push_str("'\n");
            }
            Self::Local(l) => l.render_into(target, indent),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SType {
    Simple(Identifier),
    Parametric(Vec<SType>, Identifier),
    Function(StackEffect),
    TypeVar(Symbol),
}

impl Renderable for SType {
    fn render_into(&self, target: &mut String, indent: usize) {
        match self {
            Self::Simple(id) => target.push_str(&id.to_string()),
            Self::Parametric(params, id) => {
                target.push_str("[");
                target.push_str(
                    &params
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                target.push_str("]");
                target.push_str(&id.to_string())
            }
            Self::Function(f) => f.render_into(target, indent),
            Self::TypeVar(t) => target.push_str(&t.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StackEffect {
    pub before: StackImage,
    pub after: StackImage,
}

impl Renderable for StackEffect {
    fn render_into(&self, target: &mut String, _: usize) {
        target.push_str("(");
        target.push_str(&self.before.to_string());
        target.push_str(" -- ");
        target.push_str(&self.after.to_string());
        target.push_str(")");
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StackImage {
    pub stack_var: Symbol,
    pub stack: Vec<SType>,
}

impl Renderable for StackImage {
    fn render_into(&self, target: &mut String, _: usize) {
        target.push_str(&self.stack_var.to_string());
        target.push_str(" ");
        target.push_str(
            &self
                .stack
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}

static IMG_VARIABLE_INDEX: AtomicUsize = AtomicUsize::new(0);

impl StackImage {
    pub fn unique_image_var() -> Symbol {
        let idx = IMG_VARIABLE_INDEX.fetch_add(1, Ordering::Relaxed);
        return Symbol(format!("@_{}", idx));
    }

    pub fn reset_index() {
        IMG_VARIABLE_INDEX.store(0, Ordering::Relaxed)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
// A function call is just the name of the function.
pub struct FunCallExpr {
    pub id: Identifier,
    pub type_args: Option<Vec<SType>>,
}

impl Renderable for FunCallExpr {
    fn render_into(&self, target: &mut String, indent: usize) {
        self.indent(target, indent);
        match &self.type_args {
            Some(tas) => {
                target.push_str("[");
                target.push_str(
                    &tas.iter()
                        .map(|ta| ta.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                target.push_str("]")
            }
            None => (),
        }
        target.push_str(&self.id.to_string());
        target.push_str("\n");
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ListExpr {
    pub value_type: SType,
    pub values: Vec<Vec<Expr>>,
}

impl Renderable for ListExpr {
    fn render_into(&self, target: &mut String, indent: usize) {
        self.indent(target, indent);
        target.push_str("list[");
        target.push_str(&self.value_type.to_string());
        target.push_str("](\n");
        for vs in &self.values {
            self.indent(target, indent + 1);
            target.push_str("val {\n");
            for v in vs {
                v.render_into(target, indent + 2)
            }
            self.indent(target, indent + 1);
            target.push_str("}\n")
        }
        self.indent(target, indent);
        target.push_str(")\n")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MapExpr {
    pub key_type: SType,
    pub value_type: SType,
    pub values: Vec<(Vec<Expr>, Vec<Expr>)>,
}

impl Renderable for MapExpr {
    fn render_into(&self, target: &mut String, indent: usize) {
        self.indent(target, indent);
        target.push_str("map[key: ");
        target.push_str(&self.value_type.to_string());
        target.push_str(", val: ");
        target.push_str(&self.value_type.to_string());
        target.push_str("](\n");
        for pair in &self.values {
            self.indent(target, indent + 1);
            target.push_str("pair {\n");
            self.indent(target, indent + 2);
            target.push_str("key: {\n");
            for v in &pair.0 {
                v.render_into(target, indent + 3)
            }
            self.indent(target, indent + 2);
            target.push_str("}, val: {\n");
            for v in &pair.1 {
                v.render_into(target, indent + 3)
            }
            self.indent(target, indent + 2);
            target.push_str("}\n");
            self.indent(target, indent + 1);
            target.push_str("}\n")
        }
        self.indent(target, indent);
        target.push_str(")\n")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CondExpr {
    pub true_block: Vec<Expr>,
    pub false_block: Vec<Expr>,
}

impl Renderable for CondExpr {
    fn render_into(&self, target: &mut String, indent: usize) {
        self.indent(target, indent);
        target.push_str("if\n");
        for t in &self.true_block {
            t.render_into(target, indent + 1)
        }
        self.indent(target, indent);
        target.push_str("else\n");
        for f in &self.false_block {
            f.render_into(target, indent + 1)
        }
        self.indent(target, indent);
        target.push_str("end\n");
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoopExpr {
    pub body: Vec<Expr>,
}

impl Renderable for LoopExpr {
    fn render_into(&self, target: &mut String, indent: usize) {
        self.indent(target, indent);
        target.push_str("loop\n");
        for b in &self.body {
            b.render_into(target, indent + 1)
        }
        self.indent(target, indent);
        target.push_str("end\n")
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MethodCallExpr {
    pub sym: Symbol,
}

impl Renderable for MethodCallExpr {
    fn render_into(&self, target: &mut String, indent: usize) {
        self.indent(target, indent);
        target.push_str("<- ");
        target.push_str(&self.sym.to_string());
        target.push_str("\n");
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockExpr {
    pub effect: StackEffect,
    pub body: Vec<Expr>,
}

impl Renderable for BlockExpr {
    fn render_into(&self, target: &mut String, indent: usize) {
        self.indent(target, indent);
        target.push_str("[[\n");
        self.indent(target, indent + 1);
        target.push_str(&self.effect.to_string());
        target.push_str("\n");
        for b in &self.body {
            b.render_into(target, indent + 1);
        }
        self.indent(target, indent);
        target.push_str("]]\n")
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LocalExpr {
    pub name: Symbol,
}

impl Renderable for LocalExpr {
    fn render_into(&self, target: &mut String, indent: usize) {
        self.indent(target, indent);
        target.push_str("local ");
        target.push_str(&self.name.to_string());
        target.push_str("\n")
    }
}
