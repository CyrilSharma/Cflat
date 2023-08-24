use crate::ir::Statement;

// A node with this index will be treated specially.
pub const INVALID: u32 = 1e9 as u32;
#[derive(Clone)]
pub struct Node {
    pub stmts: Vec<Box<Statement>>,
    pub t: Option<usize>,
    pub f: Option<usize>
}
impl Node {
    pub fn new() -> Self {
        Node { 
            stmts: Vec::new(), 
            t: None, f: None
        }
    }
}
pub struct CFG { 
    pub nodes:  Vec<Node>,
    pub starts: Vec<usize>
}