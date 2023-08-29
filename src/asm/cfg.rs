use super::asm::*;
use crate::registry::Registry;
use std::mem::MaybeUninit;

#[derive(Clone)]
pub struct Node {
    // AA is so small it's cheaper to just copy it.
    pub asm: AA,
    pub t:   Option<usize>,
    pub f:   Option<usize>
}
pub struct CFG { 
    pub order:  Vec<usize>,
    pub nodes:  Vec<Node>,
    pub start:  usize
}

impl CFG {
    #[allow(invalid_value)]
    pub fn build(r: &Registry, stmts: Vec<AA>) -> CFG {
        // Alternative is Option<AA>, but it's only optional at build time...
        let mut nodes: Vec<Node> = unsafe {
            vec![
                MaybeUninit::uninit().assume_init();
                r.nlabels as usize
            ]
        };
        let mut order: Vec<usize> = Vec::new();
        let mut iter = stmts.into_iter().peekable();
        let mut cur = nodes.len();
        while let Some(stmt) = iter.next() {
            use AA::*;
            // If we're not at a Label, make a new node.
            if !matches!(stmt, Label(_)) {
                nodes.push(Node { asm: stmt, t: None, f: None });
                cur = nodes.len() - 1;
                order.push(cur);
            }
            match stmt {
                B(b)     => nodes[cur].t = Some(b as usize),
                Label(b) => {
                    order.push(b as usize);
                    nodes[b as usize].asm = stmt;
                    let Some(pk) = iter.peek() else { continue };
                    if let Label(l) = *pk { 
                        nodes[b as usize].f = Some(l as usize);
                    } else {
                        nodes[b as usize].f = Some(nodes.len());
                    }
                    nodes[b as usize].t = None;
                },
                CBZ(b) | CBNZ(b) => {
                    nodes[cur].t = Some(b as usize);
                    let Some(pk) = iter.peek() else { continue };
                    if let Label(l) = *pk { 
                        nodes[cur].f = Some(l as usize);
                    } else {
                        nodes[cur].f = Some(nodes.len());
                    }
                },
                Ret => (),
                _ => {
                    let Some(pk) = iter.peek() else { continue };
                    if let Label(l) = *pk { 
                        nodes[cur].f = Some(l as usize);
                    } else {
                        nodes[cur].f = Some(nodes.len());
                    }
                }
            }
        }
        assert!(order.len() == nodes.len());
        return CFG { order, nodes, start: 0 }
    }
    pub fn export(self) -> Vec<AA> {
        let mut res: Vec<AA> = Vec::new();
        for idx in self.order.clone() {
            res.push(self.nodes[idx].asm);
        }
        return res;
    }
}