use super::asm::*;
use crate::registry::Registry;

#[derive(Clone)]
pub struct Node {
    // AA is so small it's cheaper to just copy it.
    pub asm: Vec<AA>,
    pub t: Option<usize>,
    pub f: Option<usize>
}
impl Node {
    pub fn new() -> Self {
        Node { 
            asm: Vec::new(), 
            t: None, f: None
        }
    }
}
pub struct CFG { 
    pub nodes:  Vec<Node>,
    pub starts: Vec<usize>
}

pub fn build(r: &Registry, stmts: Vec<AA>) -> CFG {
    let mut nodes  = vec![Node::new(); r.nlabels as usize];
    let starts = (0..(r.nfuncs as usize)).collect();
    let mut iter = stmts.into_iter().peekable();
    let mut cur = nodes.len();
    while let Some(stmt) = iter.next() {
        use AA::*;
        match stmt {
            Label(b) => {
                cur = b as usize;
                nodes[cur].stmts.push(stmt);
            },
            B(b) | BL(b) => {
                nodes[cur].asm.push(stmt);
                nodes[cur].t = Some(b as usize);
                let Some(pk) = iter.peek() else { continue };
                if let Label(l) = *pk {
                    cur = l as usize;
                } else {
                    nodes.push(Node::new());
                    cur = nodes.len() - 1;
                }
            },
            CBZ(b) | CBNZ(b) => {
                nodes[cur].asm.push(stmt);
                nodes[cur].t = Some(b as usize);
                let Some(pk) = iter.peek() else { continue };
                if let Label(l) = *pk { 
                    nodes[cur].f = Some(l as usize);
                    cur = l as usize;
                } else {
                    nodes[cur].f = Some(nodes.len());
                    nodes.push(Node::new());
                    cur = nodes.len() - 1;
                }
            },
            Ret => nodes[cur].asm.push(stmt),
            _ => {
                nodes[cur].asm.push(stmt);
                let Some(pk) = iter.peek() else { continue };
                if let Label(l) = *pk { 
                    nodes[cur].f = Some(l as usize);
                    cur = l as usize;
                }
            },
        }
    }
    return CFG { nodes, starts }
}