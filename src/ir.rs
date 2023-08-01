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

#[derive(Debug, Clone)]
pub enum Primitive {
    Int(i32),
    Float(f32)
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