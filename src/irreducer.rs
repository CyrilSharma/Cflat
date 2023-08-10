use crate::ir::*;
use crate::registry::Registry;

use bumpalo::Bump;
pub struct Reducer<'a> {
    arena: &'a mut Bump,
    reg: &'a mut Registry
}
impl<'a> Reducer<'a> {
    pub fn new(arena: &'a mut Bump, reg: &mut Registry) -> Self {
        reg.ret = reg.nids;
        reg.nids += 1;
        return Self { arena, reg }
    }
    pub fn reduce(&mut self, stmts: Vec<&mut Statement>)
        -> Vec<&mut Statement> {
        return self.seq(stmts);
    }
    fn statement(&mut self, s: &mut Statement) -> Vec<&mut Statement> {
        use Statement::*;
        match s {
            Expr(e) => self.expr_statement(e),
            CJump(e, t, f) => {
                let (mut s1, e1) = self.expression(e);
                s1.push(self.arena.alloc(CJump(e1, *t, *f)));
                return s1;
            },
            Jump(_) | Label(_) => return vec![s],
            Return(r)  => match r {
                None => return vec![self.arena.alloc(Return(None))],
                Some(e) => {
                    let (mut s1, e1) = self.expression(e);
                    s1.push(self.arena.alloc(Return(Some(e1))));
                    return s1;
                }
            }
            Move(d, s) => return self._move(d, s),
            Seq(s) => return self.seq(*s),
        }
    }
    fn expr_statement(&mut self, e: &mut Expr) -> Vec<&mut Statement> {
        let (mut s1, e1) = self.expression(e);
        match *e1 {
            Expr::Call(_, _) => s1.push(self.arena.alloc(
                Statement::Expr(e1)
            )),
            _ => ()
        }
        return s1;
    }
    fn _move(&mut self, d: &mut Expr, s: &mut Expr) -> Vec<&mut Statement> {
        use Expr::*;
        match d {
            Temp(_) => {
                let (mut s1, e1) = self.expression(s);
                s1.push(self.arena.alloc(
                    Statement::Move(d, e1)
                ));
                return s1;
            }
            Mem(_) => {
                /* TODO: check e1 & e2 commute? */
                let id = self.create_temp();
                let mut v = Vec::<&mut Statement>::new();
                let (sl, el) = self.expression(d);
                let (sr, er) = self.expression(s);
                v.extend(sl);
                v.push(self.arena.alloc(Statement::Move(
                    self.arena.alloc(Expr::Temp(id)),
                    el,
                )));
                v.extend(sr);
                v.push(self.arena.alloc(Statement::Move(
                    self.arena.alloc(Temp(id)),
                    er
                )));
                return v;
            }
            _ => unreachable!()
        }
    }
    fn seq(&mut self, stmts: Vec<&mut Statement>) -> Vec<&mut Statement> {
        let mut v = Vec::<&mut Statement>::new();
        for s in stmts {
            v.extend(self.statement(s));
        }
        return v;
    }
    fn expression(&mut self, e: &mut Expr)
        -> (Vec<&mut Statement>, &mut Expr) {
        use Expr::*;
        match e {
            Mem(e1) | Address(e1) => {
                let (v, e2) = self.expression(e1);
                return (v, self.arena.alloc(Mem(e2)))
            },
            UnOp(op, e)        => return self.unary(*op, e),
            BinOp(l, op, r)    => return self.binary(l, *op, r),
            Call(l, exprs)     => return self.call(*l, exprs),
            ESeq(s, e)         => return self.eseq(s, e),
            Const(_) | Temp(_) => return (Vec::new(), e)
        }
    }
    fn unary(&mut self, op: Operator, e: &mut Expr)
        -> (Vec<&mut Statement>, &mut Expr) {
        let (s1, e1) = self.expression(e);
        let e = self.arena.alloc(Expr::UnOp(op, e1));
        return (s1, e);
    }
    fn binary(&mut self, l: &mut Expr, op: Operator, r: &mut Expr)
        -> (Vec<&mut Statement>, &mut Expr) {
        // This can be made more efficient.
        // You need to be able to prove whether the expression commute.
        let mut v = Vec::<&mut Statement>::new();
        let (sl, el) = self.expression(l);
        let (sr, er) = self.expression(r);
        let id = self.create_temp();
        v.extend(sl);
        v.push(self.arena.alloc(Statement::Move(
            self.arena.alloc(Expr::Temp(id)), el
        )));
        v.extend(sr);
        let e = Expr::BinOp(
            self.arena.alloc(Expr::Temp(id)),
            op, er
        );
        return (v, self.arena.alloc(e));
    }
    fn call(&mut self, l: Label, exprs: &Vec<&mut Expr>)
        -> (Vec<&mut Statement>, &mut Expr) {
        use Expr::*;
        let mut temps = Vec::<&mut Expr>::new();
        let mut v = Vec::<&mut Statement>::new();
        for e1 in exprs {
            let (s2, e2) = self.expression(*e1);
            v.extend(s2);
            let i = self.create_temp();
            v.push(self.arena.alloc(Statement::Move(
                self.arena.alloc(Temp(i)), e2
            )));
            temps.push(self.arena.alloc(Temp(i)));
        }
        v.push(self.arena.alloc(Statement::Expr(
            self.arena.alloc(Call(l, temps))
        )));
        let id = self.create_temp();
        v.push(self.arena.alloc(Statement::Move(
            self.arena.alloc(Temp(id)),
            self.arena.alloc(Temp(self.reg.ret))
        )));
        return (v, self.arena.alloc(Temp(id)));
    }
    fn eseq(&mut self, s: &mut Statement, e: &mut Expr)
        -> (Vec<&mut Statement>, &mut Expr) {
        let mut s1 = self.statement(s);
        let (s2, e1) = self.expression(e);
        s1.extend(s2);
        return (s1, e1);
    }
    #[allow(dead_code, unused_variables)] // Do when brain big
    fn commute(l: &mut Expr, r: &mut Expr) -> bool { todo!() }
    fn create_temp(&mut self) -> u32 {
        self.reg.nids += 1;
        return self.reg.nids - 1;
    }
}