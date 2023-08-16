use std::collections::HashMap;
use crate::ir::*;
use crate::cfg::*;
pub struct CfgBuilder {
    nid: usize, // the current block
    link: bool, // whether prev & cur should be connectd.
    lookup: HashMap<u32, usize>,
    nodes: Vec<Node>,
    start: Option<usize>,
}

impl CfgBuilder {
    pub fn new() -> CfgBuilder  { 
        CfgBuilder {
            nid: 0,
            link: true,
            lookup: HashMap::new(),
            nodes: Vec::new(),
            start: None,
        }
    }
    pub fn build(&mut self, stmts: &[Box<Statement>]) -> CFG {
        use Statement::*;
        self.create_node();
        for stmt in stmts {
            let ns = &self.nodes[self.nid].stmts;
            if let Label(_) = **stmt { ns.push(*stmt); }
            match **stmt {
                Expr(_) | Seq(_)     => unreachable!(),
                Jump(l)              => self.jump(l),
                CJump(e, l1, l2)     => self.cjump(&e, l1, l2),
                Label(l)             => self.label(l),
                Return(_)            => self.link = false,
                _                    => self.link = true,
            }
        }
        return CFG { 
            nodes: std::mem::take(&mut self.nodes),
            start: self.start.unwrap(),
        }
    }
    fn jump(&mut self, l: Label) {
        let id = self.get(l);
        self.nodes[self.nid].t = Some(id);
        self.link = false;
        self.nid = self.create_node();
    }
    fn cjump(&mut self, e: &Expr, l1: Label, l2: Label) {
        let id1 = self.get(l1);
        self.nodes[self.nid].t = Some(id1);
        let id2 = self.get(l2);
        self.nodes[self.nid].f = Some(id2);
        self.link = false;
        self.nid = self.create_node();
    }
    fn label(&mut self, l: Label) {
        let old = self.nid;
        self.nid = self.get(l);
        if l == 0 { self.start = Some(self.nid); }
        if self.link {
            self.nodes[old].t = Some(self.nid);
        }
    }
    fn get(&mut self, i: u32) -> usize {
        if let Some(id) = self.lookup.get(&i) { return *id; }
        self.lookup.insert(i, self.nodes.len());
        return self.create_node();
    }
    fn create_node(&mut self) -> usize {
        self.nodes.push(Node { 
            stmts: Vec::new(), 
            t: None, f: None
        });
        return self.nodes.len() - 1;
    }
}