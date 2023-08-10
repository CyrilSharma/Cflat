use std::mem;

pub type Label = u32;
// Ugly but less ugly then &'l mut Expr<'l>
// We mark these mutable to prevent accidental cloning.
pub type RefE<'l>  = &'l mut Expr<'l>;
pub type RefS<'l>  = &'l mut Statement<'l>;

pub enum Expr<'l> {
    Const(Primitive),
    Temp(u32), /* ID */
    UnOp(Operator, RefE<'l>),
    BinOp(RefE<'l>, Operator, RefE<'l>),
    Mem(RefE<'l>),
    Call(Label, Vec<RefE<'l>>),
    Address(RefE<'l>), /* Temp, Access */
    ESeq(RefS<'l>, RefE<'l>)
}
impl<'l> Expr<'l> {
    pub fn addr(&self) -> usize {
        unsafe { mem::transmute(self) }
    }
}

pub enum Statement<'l> {
    Expr(RefE<'l>),
    Move(RefE<'l>, RefE<'l>),
    Seq(Vec<RefS<'l>>),
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