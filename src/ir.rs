use std::mem;
pub type Label = u32;
#[derive(Clone)]
pub enum Expr {
    Const(Primitive),
    Temp(u32), /* ID */
    UnOp(Operator, Box<Expr>),
    BinOp(Box<Expr>, Operator, Box<Expr>),
    Mem(Box<Expr>),
    Call(Label, Vec<Expr>),
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
    Seq(Vec<Statement>),
    Jump(Label),
    CJump(Box<Expr>, Label, Label),
    Label(Label),
    Return(Option<Box<Expr>>)
}
impl Statement {
    pub fn addr(&self) -> usize {
        unsafe { mem::transmute(self) }
    }
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Int(i32),
    Float(f32)
}
impl Primitive {
    pub fn bits(&self) -> usize {
        use Primitive::*;
        return match self {
            Int(i)   => unsafe { mem::transmute(i) },
            Float(f) => unsafe { mem::transmute(f) },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Neg,
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
    Lt,
    Gt,
}