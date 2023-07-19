//--------Modules------------
pub struct Module {
    pub functions: Vec<Box<FunctionDeclaration>>
}


//--------Functions------------
pub struct FunctionDeclaration {
    pub ret:        Kind,
    pub name:       String,
    pub params:     Vec<Parameter>,
    pub statement:  Box<Statement>,
}

#[derive(Clone)]
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
pub struct Expr {
    pub etype: ExprType,
    pub kind: Option<Kind>
}
impl Expr {
    fn new(e: ExprType) -> Self {
        Expr {
            etype: e,
            kind: None
        }
    }
}

pub enum ExprType {
    Function(FunctionCall),
    Access(AccessExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Integer(i32),
    Float(f32),
    Identifier(Identifier),
}

pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Box<Expr>>,
    pub kind: Option<Kind>,
    pub id:   u32
}

pub struct AccessExpr {
    pub name:   String,
    pub offset: Box<Expr>,
    pub kind:   Option<Kind>,
}

pub struct UnaryExpr {
    pub unary_op: UnaryOp,
    pub expr:     Box<Expr>,
    pub kind:     Option<Kind>
}

pub struct BinaryExpr {
    pub binary_op: BinaryOp,
    pub left:      Box<Expr>,
    pub right:     Box<Expr>,
    pub kind:      Option<Kind>
}

pub struct Identifier {
    pub name: String,
    pub kind: Option<Kind>,
    pub id:   u32
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
    Assign
}

#[derive(Debug)]
pub enum UnaryOp {
    Star,
    Not,
    Neg,
    Address
}

#[derive(Debug, PartialEq, Eq)]
pub enum JumpOp {
    Continue,
    Return,
    Break
}

//--------Types------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Kind {
    pub indir: u32,
    pub prim: Primitive
}
impl Kind {
    pub fn void() -> Self {
        Self {
            indir: 0,
            prim: Primitive::Void
        }
    }
    pub fn int() -> Self {
        Self {
            indir: 0,
            prim: Primitive::Integer
        }
    }
    pub fn float() -> Self {
        Self {
            indir: 0,
            prim: Primitive::Float
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Primitive {
    Void,
    Integer,
    Float
}