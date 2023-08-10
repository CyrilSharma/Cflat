use crate::ir::{Statement, self};
#[derive(Clone)]
pub struct Node<'l> {
    pub stmts: Vec<Statement<'l>>,
    pub t: Option<usize>,
    pub f: Option<usize>,
}
pub struct CFG<'l> { 
    pub nodes: Vec<Node<'l>>,
    pub start: usize
}