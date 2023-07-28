use std::fmt;
use crate::utils::polymorphic_enum;

//--------Modules------------
pub struct Module {
    pub functions: Vec<Box<FunctionDeclaration>>
}


//--------Functions------------
pub struct FunctionDeclaration {
    pub ret:        Kind,
    pub name:       String,
    pub params:     Vec<Parameter>,
    pub statement:  Box<CompoundStatement>,
    pub id:         u32
}

#[derive(Clone)]
pub struct Parameter {
    pub kind: Kind,
    pub name: String
}

//--------Statements------------
pub enum Statement {
    Declare(DeclareStatement),
    Expr(ExprStatement),
    If(IfStatement),
    For(ForStatement),
    While(WhileStatement),
    Compound(CompoundStatement),
    Jump(JumpStatement)
}

pub struct DeclareStatement {
    pub id: u32,
    pub kind: Kind,
    pub name: String,
    pub val:  Option<Box<Expr>>
}

pub struct ExprStatement {
    pub expr: Option<Box<Expr>>
}

pub struct IfStatement {
    pub condition:  Box<Expr>,
    pub true_stmt:  Box<Statement>,
    pub false_stmt: Option<Box<Statement>>
}

pub struct ForStatement {
    pub init:   Box<ExprStatement>,
    pub each:   Box<ExprStatement>,
    pub end:    Option<Box<Expr>>,
    pub stmt:   Box<Statement>
}

pub struct WhileStatement {
    pub condition: Box<Expr>,
    pub stmt:      Box<Statement>
}

pub struct CompoundStatement {
    pub stmts: Vec<Statement>
}

pub struct JumpStatement {
    pub jump_type: JumpOp,
    pub expr:      Option<Box<Expr>>
}

//--------Expressions------------
polymorphic_enum! {
    Expr => [
        Function(FunctionCall),
        Access(AccessExpr),
        Unary(UnaryExpr),
        Binary(BinaryExpr),
        Integer(i32),
        Float(f32),
        Ident(Identifier)
    ],
    Attributes => [
        kind: Option<Kind>,
        id:   u32
    ]
}

pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expr>,
    pub kind: Option<Kind>,
    pub id:   u32
}

pub struct AccessExpr {
    pub name:   String,
    pub offset: Box<Expr>,
    pub kind:   Option<Kind>,
}

pub struct UnaryExpr {
    pub unary_op: UnaryOp,
    pub expr:     Box<Expr>,
    pub kind:     Option<Kind>
}

pub struct BinaryExpr {
    pub binary_op: BinaryOp,
    pub left:      Box<Expr>,
    pub right:     Box<Expr>,
    pub kind:      Option<Kind>
}

pub struct Identifier {
    pub name: String,
    pub kind: Option<Kind>,
    pub id:   u32
}

//--------Operations------------
#[derive(Debug)]
pub enum BinaryOp {
    Mul,
    Div,
    Add,
    Sub,
    Leq,
    Geq,
    Lt,
    Gt,
    Eq,
    Peq,
    Teq,
    Deq,
    Seq,
    Neq,
    Or,
    And,
    Assign
}

#[derive(Debug)]
pub enum UnaryOp {
    Star,
    Not,
    Neg,
    Address
}

#[derive(Debug, PartialEq, Eq)]
pub enum JumpOp {
    Continue,
    Return,
    Break
}

//--------Types------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Kind {
    pub indir: u32,
    pub prim: Primitive
}
impl Kind {
    pub fn void() -> Self {
        Self {
            indir: 0,
            prim: Primitive::Void
        }
    }
    pub fn int() -> Self {
        Self {
            indir: 0,
            prim: Primitive::Integer
        }
    }
    pub fn float() -> Self {
        Self {
            indir: 0,
            prim: Primitive::Float
        }
    }
}
impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.prim, "*".repeat(self.indir as usize))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Primitive {
    Void,
    Integer,
    Float
}
impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 
            match self {
                Primitive::Void => "void",
                Primitive::Integer => "int",
                Primitive::Float => "float"
            }
        )
    }
}