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

mod identifiers;
mod twist;
pub use identifiers::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Sect {
    pub uses: Vec<UseDecl>,
    pub defs: Vec<Definition>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UseDecl {
    pub sect: ModulePath,
    pub type_names: Vec<UpperName>,
    pub function_names: Vec<LowerName>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SchismType {
    ParametricType(TypeIdent, Vec<Box<SchismType>>),
    SimpleType(TypeIdent),
    UnboundTypeVar(TypeVarName),
    FunctionType(StackEffect),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum StackEntry {
    TypeEntry(Box<SchismType>),
    NamedEntry(LowerName, Box<SchismType>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StackImage {
    pub context: Option<ContextName>,
    pub stack: Vec<StackEntry>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StackEffect {
    pub effect_domains: Vec<TypeIdent>,
    pub before: StackImage,
    pub after: StackImage,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Definition {
    Sig(SigDef),
    Obj(ObjectDef),
    Function(FunctionDef),
    Var(VarDef),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SigDef {
    pub name: UpperName,
    pub type_params: Vec<TypeParam>,
    pub composes: Vec<SchismType>,
    pub operations: Vec<OperationSig>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypeParam {
    pub name: TypeVarName,
    pub constraint: Option<SchismType>,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ObjectDef {
    pub name: UpperName,
    pub type_params: Vec<TypeParam>,
    pub composes: Vec<SchismType>,
    pub inputs: StackImage,
    pub members: Vec<ObjectMemberDecl>,
    pub body: Vec<Statement>,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionDef {
    pub name: LowerName,
    pub type_params: Vec<TypeParam>,
    pub effect: StackEffect,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VarDef {
    pub name: LowerName,
    pub value_type: SchismType,
    pub inputs: StackImage,
    pub body: Vec<Statement>,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CondClause {
    pub condition: Box<Statement>,
    pub body: Vec<Box<Statement>>,
}
