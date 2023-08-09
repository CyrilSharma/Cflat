use std::mem;

pub type Label = u32;
pub type RefE<'l>  = &'l Expr<'l>;

#[derive(Clone)]
pub enum Expr<'l> {
    Const(Primitive),
    Temp(u32), /* ID */
    UnOp(Operator, RefE<'l>),
    BinOp(RefE<'l>, Operator, RefE<'l>),
    Mem(RefE<'l>),
    Call(Label, Vec<Expr<'l>>),
    Address(RefE<'l>), /* Temp, Access */
    ESeq(&'l Statement<'l>, RefE<'l>)
}
impl<'l> Expr<'l> {
    pub fn addr(&self) -> usize {
        unsafe { mem::transmute(self) }
    }
}

#[derive(Clone)]
pub enum Statement<'l> {
    Expr(RefE<'l>),
    Move(RefE<'l>, RefE<'l>),
    Seq(Vec<Statement<'l>>),
    Jump(Label),
    CJump(RefE<'l>, Label, Label),
    Label(Label),
    Return(Option<RefE<'l>>)
}
impl<'l> Statement<'l> {
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