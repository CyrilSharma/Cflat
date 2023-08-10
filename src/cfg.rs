use crate::ir::Statement;
pub struct Node<'l> {
    pub stmts: Vec<&'l mut Statement<'l>>,
    pub t: Option<usize>,
    pub f: Option<usize>,
}
pub struct CFG<'l> { 
    pub nodes: Vec<Node<'l>>,
    pub start: usize
}