pub enum Expr {
    Const(Primitive),
    Temp(u32), /* ID */
    UnOp(Operator, Box<Expr>),
    BinOp(Box<Expr>, Operator, Box<Expr>),
    Mem(Box<Expr>),
    Call(Label, Vec<Expr>),
    Name(Label),
    Address(u32), /* ID */
    ESeq(Box<Statement>, Box<Expr>)
}

pub enum Statement {
    Expr(Box<Expr>),
    Move(Box<Expr>, Box<Expr>),
    Seq(Vec<Statement>),
    Jump(Expr),
    CJump(Box<Expr>, Label, Label),
    Label(Label),
    Return(Option<Box<Expr>>)
}

#[derive(Debug, Clone, Copy)]
pub struct Label {
    pub id: u32
}

#[derive(Debug)]
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