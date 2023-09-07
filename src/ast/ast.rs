use std::fmt;
use Primitive::*;

//--------Modules------------
pub struct Module {
    pub functions: Vec<Box<FunctionDeclaration>>,
}

//--------Functions------------
pub struct FunctionDeclaration {
    pub ret: Kind,
    pub name: String,
    pub params: Vec<Parameter>,
    pub stmt: Box<Statement>,
    pub id: u32,
}

#[derive(Clone)]
pub struct Parameter {
    pub kind: Kind,
    pub name: String,
    pub id:   u32
}

//--------Statements------------
pub enum Statement {
    Declare(DeclareStatement),
    Expr(ExprStatement),
    If(IfStatement),
    For(ForStatement),
    While(WhileStatement),
    Compound(CompoundStatement),
    Jump(JumpStatement),
}

pub struct DeclareStatement {
    pub id: u32,
    pub kind: Kind,
    pub name: String,
    pub val: Option<Box<Expr>>,
}

pub struct ExprStatement {
    pub expr: Option<Box<Expr>>,
}

pub struct IfStatement {
    pub condition: Box<Expr>,
    pub true_stmt: Box<Statement>,
    pub false_stmt: Option<Box<Statement>>,
}

pub struct ForStatement {
    pub init: Box<Statement>,
    pub cond: Option<Box<Expr>>,
    pub each: Option<Box<Expr>>,
    pub stmt: Box<Statement>,
}

pub struct WhileStatement {
    pub condition: Box<Expr>,
    pub stmt: Box<Statement>,
}

pub struct CompoundStatement {
    pub stmts: Vec<Statement>,
}

pub struct JumpStatement {
    pub jump_type: JumpOp,
    pub expr: Option<Box<Expr>>,
}

//--------Expressions------------
pub enum Expr {
    Function(FunctionCall),
    Access(AccessExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Integer(i64),
    Float(f64),
    Ident(Identifier),
}
impl Expr {
    pub fn kind(&self) -> Option<Kind> {
        use Expr::*;
        match self {
            Function(i) => i.kind,
            Access(i) => i.kind,
            Unary(i) => i.kind,
            Binary(i) => i.kind,
            Integer(_) => Some(Kind::int()),
            Float(_) => Some(Kind::float()),
            Ident(i) => i.kind,
        }
    }
    pub fn id(&self) -> u32 {
        use Expr::*;
        match self {
            Access(i) => i.id,
            Ident(i) => i.id,
            _ => panic!("Only Access & Ident have ID!"),
        }
    }
}

pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expr>,
    pub kind: Option<Kind>,
    pub id: u32,
}

pub struct AccessExpr {
    pub name: String,
    pub offsets: Vec<Expr>,
    pub sizes: Vec<u32>,
    pub kind: Option<Kind>,
    pub id: u32,
}

pub struct UnaryExpr {
    pub unary_op: UnaryOp,
    pub expr: Box<Expr>,
    pub kind: Option<Kind>,
}

pub struct BinaryExpr {
    pub binary_op: BinaryOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub kind: Option<Kind>,
}

pub struct Identifier {
    pub name: String,
    pub kind: Option<Kind>,
    pub id: u32,
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
    Assign,
}

#[derive(Debug)]
pub enum UnaryOp {
    Star,
    Not,
    Neg,
    Address,
}

#[derive(Debug, PartialEq, Eq)]
pub enum JumpOp {
    Continue,
    Return,
    Break,
}

//--------Types------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Kind {
    pub indir: u32,
    pub prim: Primitive,
}
impl Kind {
    pub fn void() -> Self {
        Self {
            indir: 0,
            prim: Void,
        }
    }
    pub fn int() -> Self {
        Self {
            indir: 0,
            prim: Int,
        }
    }
    pub fn float() -> Self {
        Self {
            indir: 0,
            prim: Float,
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
    Int,
    Float,
}
impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Void => "void",
            Int => "int",
            Float => "float",
        })
    }
}
