//--------Modules------------
pub struct Module {
    pub functions: Vec<Box<FunctionDeclaration>>
}


//--------Functions------------
pub struct FunctionDeclaration {
    pub ret:        Kind,
    pub name:       String,
    pub params:     Option<Vec<Parameter>>,
    pub statement:  Box<Statement>,
}
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
    pub init:   Option<Box<Expr>>,
    pub each:   Option<Box<Expr>>,
    pub end:    Option<Box<Expr>>,
    pub stmt:   Box<Statement>
}

pub struct WhileStatement {
    pub condition: Box<Expr>,
    pub stmt:      Box<Statement>
}

pub struct CompoundStatement {
    pub stmts: Option<Vec<Box<Statement>>>
}

pub struct JumpStatement {
    pub jump_type: JumpOp,
    pub expr:      Option<Box<Expr>>
}

//--------Expressions------------
pub enum Expr {
    Function(FunctionCall),
    Access(AccessExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Integer(i32),
    Float(f32),
    Identifier(String),
}

pub struct FunctionCall {
    pub name: String,
    pub args: Option<Vec<Box<Expr>>>
}

pub struct AccessExpr {
    pub name: String,
    pub offset: Box<Expr>
}

pub struct UnaryExpr {
    pub unary_op: UnaryOp,
    pub expr:     Box<Expr>
}

pub struct BinaryExpr {
    pub binary_op: BinaryOp,
    pub left:      Box<Expr>,
    pub right:     Box<Expr>,
}

//--------Operations------------
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

pub enum UnaryOp {
    Star,
    Not,
    Neg
}

pub enum JumpOp {
    Continue,
    Return,
    Break
}

//--------Types------------
pub struct Kind {
    pub indirection: Option<u32>,
    pub prim: Primitive
}
pub enum Primitive {
    Void,
    Integer,
    Float
}