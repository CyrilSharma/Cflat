use std::collections::HashMap;

use crate::ir::*;

#[derive(Clone)]
struct Node {
    // exclusive range [s...e)
    range: (usize, usize),
    edges: Vec<usize>
}
struct CFG { nodes: Vec<Node> }
struct CfgBuilder {
    nodes: Vec<Node>,
    lookup: HashMap<u32, usize>
}
impl CfgBuilder {
    fn new() -> CfgBuilder  { 
        CfgBuilder {
            nodes: Vec::new(),
            lookup: HashMap::new()
        } 
    }
    fn build(&mut self, stmts: Vec<Statement>) -> CFG {
        use Statement::*;
        let mut nid;
        let mut idx: usize = 0;
        while idx < stmts.len() {
            nid = self.nodes.len();
            self.nodes.push(Node { 
                range: (idx, idx), 
                edges: Vec::new()
            });
            while idx < stmts.len() {
                idx += 1; // counter increases even on break.
                match stmts[idx - 1] {
                    Expr(_) | Move(_, _) => {
                        self.nodes[nid].range.1 += 1;
                    },
                    Jump(l) => {
                        self.nodes[nid].range.1 += 1;
                        let id = self.find(l.id);
                        self.nodes[nid].edges.push(id);
                        break;
                    },
                    CJump(_, l1, l2) => {
                        self.nodes[nid].range.1 += 1;
                        let id1 = self.find(l1.id);
                        self.nodes[nid].edges.push(id1);
                        let id2 = self.find(l2.id);
                        self.nodes[nid].edges.push(id2);
                        break;
                    },
                    Return(_) => {
                        self.nodes[nid].range.1 += 1;
                        break;
                    },
                    Label(l) => {
                        nid = self.find(l.id);
                        self.nodes[nid].range = (idx - 1, idx);
                    }
                    _ => unreachable!()
                }
            }
        }
        return CFG {
            nodes: self.nodes
                .iter()
                .cloned()
                .filter(|n| n.range.0 != n.range.1)
                .collect()
        }
    }
    fn find(&mut self, i: u32) -> usize {
        return match self.lookup.get(&i) {
            None => {
                self.lookup.insert(i, self.nodes.len());
                self.nodes.push(Node { range: (0, 0), edges: Vec::new() });
                self.nodes.len() - 1
            }
            Some(id) => *id
        };
    }

}