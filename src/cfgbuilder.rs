use std::collections::HashMap;
use crate::ir::{self, *};
use crate::cfg::*;
use crate::registry::Registry;
pub struct Builder<'l> {
    nid: usize, // the current block
    link: bool, // whether prev & cur should be connectd.
    lookup: HashMap<u32, usize>,
    nodes: Vec<Node>,
    start: Option<usize>,
    reg: &'l mut Registry
}

impl<'l> Builder<'l> {
    pub fn new(registry: &mut Registry) -> Builder  { 
        Builder {
            nid: 0,
            link: true,
            lookup: HashMap::new(),
            nodes: Vec::new(),
            start: None,
            reg: registry
        }
    }
    pub fn build(&mut self, stmts: Vec<Box<Statement>>) -> CFG {
        use Statement::*;
        self.create_node();
        for stmt in stmts {
            let old = self.nid;
            match *stmt {
                Seq(_)               => unreachable!(),
                Jump(l)              => self.jump(l),
                CJump(_, l1, l2)     => self.cjump(l1, l2),
                Label(l)             => self.label(l),
                Return(_)            => self.link = false,
                _                    =>	self.link = true,
            }
            if !matches!(*stmt, Label(_) | Expr(_)) {
                self.nodes[old].stmts.push(stmt);
            } else if matches!(*stmt, Expr(_)) {
                use ir::Expr::Call;
                let Expr(e) = *stmt else { unreachable!() };
                let Call(c, v) = *e else { unreachable!() };
                let exp = Box::new(Call(self.get(c) as u32, v));
                let nw 	= Box::new(Expr(exp));
                self.nodes[old].stmts.push(nw);
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
    fn cjump(&mut self, l1: Label, l2: Label) {
        let id1 = self.get(l1);
        self.nodes[self.nid].t = Some(id1);
        let id2 = self.get(l2);
        self.nodes[self.nid].f = Some(id2);
        self.link = false;
        self.nid = self.create_node();
    }
    fn label(&mut self, l: Label) {
        let old = self.nid;
        let mut removed = false;
        if self.nodes[old].stmts.len() == 0 {
            self.nodes.pop();
            removed = true;
        }
        self.nid = self.get(l);
        if l == 0 { self.start = Some(self.nid); }
        if !removed && self.link {
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