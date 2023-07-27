use crate::ast;
pub enum Expr {
    Const(Primitive),
    Temp(u32), /* ID */
    UnOp(Operator, Box<Expr>),
    BinOp(Box<Expr>, Operator, Box<Expr>),
    Mem(Box<Expr>),
    Call(Label, Vec<Expr>),
    Name(Label),
}

pub enum Statement {
    Move(Box<Expr>, Box<Expr>),
    Seq(Vec<Statement>),
    Jump(Expr),
    CJump(Box<Expr>, Label, Label),
    Label(Label),
    Return(Option<Box<Expr>>)
}

pub struct Label {
    pub id: u32
}

pub enum Primitive {
    Int(i32),
    Float(f32)
}

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
    Not,
    Eq,
    Neq,
    Leq,
    Geq,
    Gt,
}