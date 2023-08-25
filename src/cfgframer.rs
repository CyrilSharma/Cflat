use crate::ir::*;
use crate::cfg::*;
use crate::registry::Registry;
use std::collections::HashMap;

pub struct Framer<'l> {
    cfg:       &'l CFG,
    frames:    Vec<HashMap<u32, usize>>,
    addressed: Vec<bool>,
    visited:   Vec<bool>,
    inc:       usize, // End of current frame.
}
impl<'l> Framer<'l> {
    pub fn new(r: &mut Registry, cfg: &'l CFG) -> Self {
        let addressed = Address::new(
            cfg, r.nids as usize
        ).address(); // nvariables.
        let visited = vec![false; r.nlabels as usize]; // r.nblocks, actually
        let frames = vec![
            HashMap::<u32, usize>::new();
            r.nfuncs as usize
        ];
        Framer {
            cfg,
            frames,
            addressed,
            visited,
            inc: 0
        }
    }
    pub fn frame(&mut self) -> Vec<HashMap<u32, usize>>{
        for ind in &self.cfg.starts {
            self.inc = 0;
            self.frame_func(*ind);
        }
        return std::mem::take(&mut self.frames);
    }
    fn frame_func(&mut self, i: usize) {
        if self.visited[i] { return }
        self.visited[i] = true;
        let n = &self.cfg.nodes[i];
        for stmt in &n.stmts {
            self.frame_stmt(&stmt);
        }
        let old = self.inc;
        if let Some(l) = n.t {
            self.frame_func(l)
        }
        self.inc = old;
        if let Some(r) = n.t {
            self.frame_func(r)
        }
    }
    fn frame_stmt(&mut self, s: &Statement) {
        use Statement::*;
        match s {
            Expr(e)    => self.frame_expr(e),
            Move(e1, e2) => {
                self.frame_expr(e1);
                self.frame_expr(e2);
            },
            Return(o) => if let Some(e) = o {
                self.frame_expr(e);
            },
            CJump(e, _, _) => self.frame_expr(e),
            Jump(_) | Label(_) => (),
            Seq(_) => unreachable!()
        }
    }
    fn frame_expr(&mut self, e: &Expr) {
        use Expr::*;
        match e {
            Const(_) => (),
            Temp(i) => {
                if self.frames[self.f].contains_key(&i) { return };
                if !self.addressed[*i as usize] { return }
                self.frames[self.f].insert(*i, self.inc);
                self.inc += 4;
                // This only works because all supported types are four bytes
                // A better language would have Temps store Types, which could
                // have varying widths.
            },
            UnOp(_, e) => self.frame_expr(e),
            BinOp(l, _, r) => {
                self.frame_expr(l);
                self.frame_expr(r);
            },
            Mem(e) => self.frame_expr(e),
            Call(_, v) => {
                for e in v { self.frame_expr(e) }
            },
            Address(e) => self.frame_expr(e),
            ESeq(_, _) => unreachable!()
        }
    }
}

struct Address<'l> {
    cfg: &'l CFG,
    addressed: Vec<bool>
}
impl<'l> Address<'l> {
    pub fn new(cfg: &'l CFG, nlabels: usize) -> Self {
        Address {
            cfg,
            addressed: vec![false; nlabels]
        }
    }
    pub fn address(&mut self) -> Vec<bool> {
        for ind in &self.cfg.starts {
            self.address_func(&self.cfg.nodes[*ind]);
        }
        return std::mem::take(&mut self.addressed);
    }
    fn address_func(&mut self, n: &Node) {
        for stmt in &n.stmts { self.address_stmt(&stmt);             }
        if let Some(l) = n.t { self.address_func(&self.cfg.nodes[l]) }
        if let Some(r) = n.t { self.address_func(&self.cfg.nodes[r]) }
    }
    fn address_stmt(&mut self, s: &Statement) {
        use Statement::*;
        match s {
            Expr(e)    => self.address_expr(e),
            Move(e1, e2) => {
                self.address_expr(e1);
                self.address_expr(e2);
            },
            Return(o) => if let Some(e) = o {
                self.address_expr(e);
            },
            CJump(e, _, _) => self.address_expr(e),
            Jump(_) | Label(_) => (),
            Seq(_) => unreachable!()
        }
    }
    fn address_expr(&mut self, e: &Expr) {
        use Expr::*;
        let Address(a) = e else { return };
        let Temp(i) = **a else { return };
        // Ignore arrays for now...
        self.addressed[i as usize] = true;
    }
}
    