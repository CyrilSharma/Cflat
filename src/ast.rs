//--------Modules------------
pub struct Module {
    functions: Vec<Box<FunctionDeclaration>>
}


//--------Functions------------
pub struct FunctionDeclaration {
    ret:        Kind,
    name:       String,
    params:     Option<Vec<Parameter>>,
    statement:  Box<Statement>,
}
pub struct Parameter {
    kind: Kind,
    name: String
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
    kind: Kind,
    name: String,
    val:  Option<Box<Expr>>
}

pub struct ExprStatement {
    expr: Option<Box<Expr>>
}

pub struct IfStatement {
    condition:  Box<Expr>,
    true_stmt:  Box<Expr>,
    false_stmt: Option<Box<Expr>>
}

pub struct ForStatement {
    init:   Option<Box<Expr>>,
    each:   Option<Box<Expr>>,
    end:    Option<Box<Expr>>,
    stmt:   Box<Statement>
}

pub struct WhileStatement {
    condition: Box<Expr>,
    stmt:      Box<Statement>
}

pub struct CompoundStatement {
    decls: Option<Vec<Box<Declaration>>>,
    stmts: Option<Vec<Box<Statement>>>
}

pub struct JumpStatement {
    jump_type: JumpOp,
    expr:     Box<Expr>
}

//--------Expressions------------
pub enum Expr {
    Integer(i32),
    Float(f32),
    Identifier(String),
    Function(FunctionCall),
    Access(AccessExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
}

pub struct FunctionCall {
    name: String,
    args: Option<Vec<Expr>>
}

pub struct AccessExpr {
    name: String,
    offset: Box<Expr>
}

pub struct UnaryExpr {
    unary_op: UnaryOp,
    expr:     Box<Expr>
}

pub struct BinaryExpr {
    binary_op: BinaryOp,
    left:      Box<Expr>,
    right:     Box<Expr>,
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
    Neq,
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
    indirection: Option<u32>,
    prim: Primitive
}
pub enum Primitive {
    Void,
    Integer,
    Float
}
