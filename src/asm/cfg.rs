use super::asm::*;
use crate::registry::Registry;

#[derive(Clone)]
pub struct Node {
    pub idx: usize,
    pub t:   Option<usize>,
    pub f:   Option<usize>
}
pub struct CFG<'l> { 
    pub asm:    &'l Vec<AA>,
    pub nodes:  Vec<Node>,
    pub start:  usize
}

impl<'l> CFG<'l> {
    #[allow(invalid_value)]
    pub fn build(r: &Registry, stmts: &'l Vec<AA>) -> CFG<'l> {
        let mut nodes: Vec<Node> = vec![
            Node { idx: usize::MAX, t: None, f: None};
            r.nlabels as usize
        ];
        let mut idx = 0;
        let mut iter = stmts.iter().peekable();
        let mut cur = nodes.len();
        let mut marked: Vec<bool> = vec![false; stmts.len()];
        while let Some(stmt) = iter.next() {
            use AA::*;
            idx += 1;
            if !matches!(stmt, Label(_)) {
                nodes.push(Node { idx: idx - 1, t: None, f: None });
                marked[idx - 1] = true;
                cur = nodes.len() - 1;
            }
            match *stmt {
                B(b)     => nodes[cur].t = Some(b as usize),
                Label(b) => {
                    nodes[b as usize].idx = idx - 1;
                    marked[idx - 1] = true;
                    let Some(pk) = iter.peek() else { continue };
                    if let Label(l) = *pk { 
                        nodes[b as usize].f = Some(*l as usize);
                    } else {
                        nodes[b as usize].f = Some(nodes.len());
                    }
                    nodes[b as usize].t = None;
                },
                CBZ(b) | CBNZ(b) => {
                    nodes[cur].t = Some(b as usize);
                    let Some(pk) = iter.peek() else { continue };
                    if let Label(l) = *pk { 
                        nodes[cur].f = Some(*l as usize);
                    } else {
                        nodes[cur].f = Some(nodes.len());
                    }
                },
                Ret => (),
                _ => {
                    let Some(pk) = iter.peek() else { continue };
                    if let Label(l) = *pk { 
                        nodes[cur].f = Some(*l as usize);
                    } else {
                        nodes[cur].f = Some(nodes.len());
                    }
                }
            }
        }
        return CFG { asm: stmts, nodes, start: 0 }
    }
}