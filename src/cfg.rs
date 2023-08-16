use crate::ir::{Statement};
#[derive(Clone)]
pub struct Node {
    pub stmts: Vec<Box<Statement>>,
    pub t: Option<usize>,
    pub f: Option<usize>
}
pub struct CFG { 
    pub nodes: Vec<Node>,
    pub start: usize
}