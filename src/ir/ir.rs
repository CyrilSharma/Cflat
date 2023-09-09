use std::mem;
use crate::asm::asm;
pub type ID = u32;
pub type Label = u32;
#[derive(Clone)]
pub enum Expr {
    Const(Primitive),
    Temp(ID),
    UnOp(Operator, Box<Expr>),
    BinOp(Box<Expr>, Operator, Box<Expr>),
    Mem(Box<Expr>),
    Call(Label, Vec<Box<Expr>>),
    Address(Box<Expr>), /* Temp, Access */
    ESeq(Box<Statement>, Box<Expr>)
}
impl Expr {
    pub fn addr(&self) -> usize {
        unsafe { mem::transmute(self) }
    }
}

#[derive(Clone)]
pub enum Statement {
    Expr(Box<Expr>),
    Move(Box<Expr>, Box<Expr>),
    Seq(Vec<Box<Statement>>),
    Jump(Label),
    CJump(Box<Expr>, Label, Label),
    Label(Label),
    Function(Label, Vec<ID>),
    Return(Option<Box<Expr>>),
    Asm(asm::AA)
}
impl Statement {
    pub fn addr(&self) -> usize {
        unsafe { mem::transmute(self) }
    }
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Int(i64),
    Float(f64)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Neg,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Not,
    Eq,
    Neq,
    Leq,
    Geq,
    Lt,
    Gt,
}