use std::collections::{BTreeMap};
use crate::ir::{self, Statement, Expr, Operator};
use crate::aa::{AA, Reg};

pub struct Translator { 
    opt:   BTreeMap<usize, (Vec<AA>, u32)>,
    count: usize
}

impl Translator {
    pub fn new(cnt: usize) -> Self { 
        Self { 
            opt: BTreeMap::new(),
            count: cnt
        }
    }
    pub fn translate(&mut self, stmts: &Vec<Statement>) -> Vec<AA> {
        let mut res = Vec::<AA>::new();
        for s in stmts {
            self.statement(s);
            let (v, _) = self.opt.get(&s.addr()).unwrap();
            res.extend(*v);
        }
        return res;
    }
    fn statement(&mut self, s: &Statement) {
        use Statement::*;
        let nid = s.addr();
        match s {
            Expr(e)        => { // only used for calls after reduction.
                let ir::Expr::Call(f, args) = **e else { unreachable!(); };
                self.call(f, args);
            }
            Move(d, s)     => self._move(d, s, nid),
            Jump(j)        => self.jump(j, nid),
            CJump(c, t, _) => self.cjump(c, *t, nid),
            Label(l)  => vec![AA::Label(*l)],
            Return(r) => vec![AA::Ret],
            _ => unreachable!()
        }
    }
    fn expression(&mut self, e: &Expr) {
        if self.opt.contains_key(e) { return }
        use Expr::*;
        use Operator::*;
        match e {
            Const(p) => self.add_label(&format!("Const ({:?})", p)),
            Temp(t) => self.add_label(&format!("Temp ({})", t)),
            UnOp(op, e) => self.unary(*op, e),
            BinOp(l, op, r) => self.binary(l, *op, r),
            Mem(m) if !self.costs.contains_key(&e.bits()) => self.mem(m),
            Address(e) => self.address(e),
            _ => unreachable!()
        }
    }
    fn _move(&mut self, d: &Expr, s: &Expr, nid: usize) {
        if self.opt.contains_key(&nid) { return; }
        use Expr::*;
        let mut best: u32 = u32::MAX;
        let mut best_asm: Vec<AA> = Vec::new();
        let test = ||{};
        match (d, s) {
            (Temp(a), Const(b))  => {
                let res = vec![AA::Mov1(Reg::ID(*a), b.bits())];
                (res, res.len() as u32)
            },
            (Temp(a), Temp(b))   => {
                let res = vec![AA::Mov2(Reg::ID(*a), Reg::ID(*b))];
                (res, res.len() as u32)
            },
            (Temp(a), Mem(T))    => {
                let Temp(i) = **T else { unreachable!() };
                let res = vec![AA::LDR2(Reg::ID(*a), Reg::ID(i))];
                (res, res.len() as u32)
            },
            (Mem(T), Const(b))   => {
                let t = self.temp();
                let Temp(reg) = **T else { unreachable!() };
                let res = vec![
                    AA::Mov1(Reg::ID(t), b.bits()),
                    AA::STR2(Reg::ID(t), Reg::ID(reg))
                ];
                (res, res.len() as u32)
            },
            (Mem(T), Temp(b))    => {
                let Temp(i) = **T else { unreachable!() };
                let res = vec![AA::STR2(Reg::ID(i), Reg::ID(*b))];
                (res, res.len() as u32)
            },
            (Mem(D), Mem(S)) => {
                let t = self.temp();
                let Temp(reg1) = **D else { unreachable!() };
                let Temp(reg2) = **S else { unreachable!() };
                let res = vec![
                    AA::LDR2(Reg::ID(t), Reg::ID(reg2)),
                    AA::STR2(Reg::ID(reg1), Reg::ID(t)),
                ];
                (res, res.len() as u32)
            }
            _ => unreachable!()
        }
        self.opt.insert(nid, (best_asm, best));
    }
    fn cjump(&mut self, j: &Expr, t: Label, f: Label) {
        let idx = self.count;
        self.add_label("Jump");
        self.add_edge(idx, self.count);
        self.expression(j);

        self.add_label(&format!("{}", t));
        self.add_edge(idx, self.count);

        self.add_label(&format!("{}", f));
        self.add_edge(idx, self.count);
    }
    fn unary(&mut self, op: Operator, e: &Expr) {
        let idx = self.count;
        self.add_label(&format!("Unary {:?}", op));
        self.add_edge(idx, self.count);
        self.expression(e);
    }
    fn binary(&mut self, l: &Expr, op: Operator, r: &Expr) {
        let idx = self.count;
        self.add_label(&format!("Binary {:?}", op));
        self.add_edge(idx, self.count);
        self.expression(l);
        self.add_edge(idx, self.count);
        self.expression(r);
    }
    fn mem(&mut self, m: &Expr) {
        let idx = self.count;
        self.add_label("Mem");
        self.add_edge(idx, self.count);
        self.expression(m);
    }
    fn call(&mut self, l: Label, v: &Vec<Expr>) {
        let idx = self.count;
        self.add_label("Call");
        self.add_edge(idx, self.count);
        self.add_label(&format!("{}", l));
        for e in v {
            self.add_edge(idx, self.count);
            self.expression(e);
        }
    }
    fn address(&mut self, e: &Expr) {
        let idx = self.count;
        self.add_label("Address");
        self.add_edge(idx, self.count);
        self.expression(e);
    }
    fn temp(&mut self) -> u32 {
        self.count += 1;
        return (self.count - 1) as u32;
    }
}