use crate::ir::*;
use crate::cfg::*;

use std::collections::HashMap;
use bumpalo::Bump;
pub struct CfgBuilder<'a> {
    nid: usize, // the current block
    link: bool, // whether prev & cur should be connectd.
    lookup: HashMap<u32, usize>,
    nodes: Vec<Node<'a>>,
    start: Option<usize>,
    arena: &'a mut Bump
}

impl<'a> CfgBuilder<'a> {
    pub fn new(arena: &mut Bump) -> CfgBuilder  { 
        CfgBuilder {
            nid: 0,
            link: true,
            lookup: HashMap::new(),
            nodes: Vec::new(),
            start: None,
            arena
        }
    }
    pub fn build(&mut self, stmts: Vec<&mut Statement>) -> CFG {
        use Statement::*;
        self.create_node();
        for stmt in stmts {
            match stmt {
                Label(_) => (),
                _ => self.nodes[self.nid].stmts.push(stmt)
            }
            match stmt {
                Expr(_) | Move(_, _) => self.link = true,
                Jump(l)              => self.jump(*l),
                CJump(e, l1, l2)     => self.cjump(&e, *l1, *l2),
                Return(r)            => self._return(*r),
                Label(l)             => self.label(*l),
                _ => unreachable!()
            }
        }
        // last node is always a return which allocates a node.
        self.nodes.pop();
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
    fn _return(&mut self, o: Option<&mut Expr>) {
        self.nid = self.create_node();
        self.link = false;
    }
    fn label(&mut self, l: Label) {
        if l == 0 { self.start = Some(self.nid); }
        let old = self.nid;
        let mut removed = false;
        if self.nodes[old].stmts.len() == 0 {
            self.nodes.pop();
            removed = true;
        }
        self.nid = self.get(l);
        if !removed && self.link {
            self.nodes[old].t = Some(self.nid);
        }
    }
    fn get(&mut self, i: u32) -> usize {
        match self.lookup.get(&i) {
            None => {
                self.lookup.insert(i, self.nodes.len());
                return self.create_node();
            }
            Some(id) => return *id
        };
    }
    fn create_node(&mut self) -> usize {
        self.nodes.push(Node { 
            stmts: Vec::new(), 
            t: None, f: None
        });
        return self.nodes.len() - 1;
    }
}