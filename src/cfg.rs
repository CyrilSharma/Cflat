use std::collections::HashMap;

use crate::ir::*;
struct Node {
    range: (usize, usize),
    edges: Vec<u32>
}

struct CFG { 
    nodes: Vec<Node>
}
impl CFG {
    fn new(stmts: Vec<Statement>) {
        use Statement::*;
        let mut nodes = Vec::<Node>::new();
        let mut lookup = HashMap::<u32, Box<Node>>::new();
        let idx: usize = 0;
        loop {
            let mut node = Node { 
                range: (idx, idx), 
                edges: Vec::new()
            };
            while idx < stmts.len() {
                match stmts[idx] {
                    Expr(_) | Move(_, _) => {
                        node.range.1 += 1;
                    }
                    Jump(_) => {

                    }
                    CJump(_, _, _) => {

                    }
                    Return(_) => {

                    }
                    Label(_) => {

                    }
                    _ => unreachable!()
                }
            }
            nodes.push(node);
        }
    }

}