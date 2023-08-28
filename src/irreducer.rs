use crate::ir::*;
use crate::registry::Registry;
pub struct Reducer<'l> {
    reg: &'l mut Registry
}
impl<'l> Reducer<'l> {
    pub fn new(registry: &'l mut Registry) -> Self {
        registry.ret = registry.nids;
        Self { reg: registry }
    }
    pub fn reduce(&mut self, stmts: Vec<Box<Statement>>)
        -> Vec<Box<Statement>> {
        return self.seq(stmts);
    }
    fn statement(&mut self, s: Box<Statement>)
        -> Vec<Box<Statement>> {
        use Statement::*;
        match *s {
            Expr(e) => self.expr_statement(e),
            CJump(e, t, f) => {
                let (mut s1, e1) = self.expression(e);
                s1.push(Box::new(CJump(e1, t, f)));
                return s1;
            },
            Jump(_) | Label(_) | Function(_, _)=> return vec![s],
            Return(r)  => match r {
                None => return vec![Box::new(Return(None))],
                Some(e) => {
                    let (mut s1, e1) = self.expression(e);
                    s1.push(Box::new(Return(Some(e1))));
                    return s1;
                }
            }
            Move(d, s) => return self._move(d, s),
            Seq(s) => return self.seq(s),
        }
    }
    fn expr_statement(&mut self, e: Box<Expr>)
        -> Vec<Box<Statement>> {
        let (mut s1, e1) = self.expression(e);
        match *e1 {
            Expr::Call(_, _) => s1.push(Box::new(
                Statement::Expr(e1)
            )),
            _ => ()
        }
        return s1;
    }
    fn _move(&mut self, d: Box<Expr>, s: Box<Expr>)
        -> Vec<Box<Statement>> {
        use Expr::*;
        match *d {
            Temp(_) => {
                let (mut s1, e1) = self.expression(s);
                s1.push(Box::new(Statement::Move(
                    d,
                    e1
                )));
                return s1;
            },
            Mem(_) => {
                /* TODO: check e1 & e2 commute? */
                let id = self.create_temp();
                let (sl, el) = self.expression(d);
                let (sr, er) = self.expression(s);
                let mut v = sl.clone();
                v.push(Box::new(Statement::Move(
                    Box::new(Expr::Temp(id)),
                    el,
                )));
                v.extend(sr);
                v.push(Box::new(Statement::Move(
                    Box::new(Temp(id)),
                    er
                )));
                return v;
            },
            _ => unreachable!()
        }
    }
    fn seq(&mut self, stmts: Vec<Box<Statement>>)
        -> Vec<Box<Statement>> {
        let mut v = Vec::<Box<Statement>>::new();
        for s in stmts {
            v.extend(self.statement(s));
        }
        return v;
    }
    fn expression(&mut self, e: Box<Expr>)
        -> (Vec<Box<Statement>>, Box<Expr>) {
        use Expr::*;
        match *e {
            Const(_) | Temp(_) => return (Vec::new(), e),
            Mem(e1) => {
                let (v, e2) = self.expression(e1);
                return (v, Box::new(Mem(e2)))
            },
            Address(e1) => {
                let (v, e2) = self.expression(e1);
                return (v, Box::new(Address(e2)))
            },
            UnOp(op, e) => return self.unary(op, e),
            BinOp(l, op, r) => return self.binary(l, op, r),
            Call(l, exprs) => return self.call(l, exprs),
            ESeq(s, e) => return self.eseq(s, e)
        }
    }
    fn unary(&mut self, op: Operator, e: Box<Expr>)
        -> (Vec<Box<Statement>>, Box<Expr>) {
        let (s1, e1) = self.expression(e);
        return (s1, Box::new(Expr::UnOp(op, e1)));
    }
    fn binary(&mut self, l: Box<Expr>, op: Operator, r: Box<Expr>)
        -> (Vec<Box<Statement>>, Box<Expr>) {
        /* This can be made more efficient if you can somehow determine if the operators commute */
        let mut v = Vec::<Box<Statement>>::new();
        let (sl, el) = self.expression(l);
        let (sr, er) = self.expression(r);
        let id = self.create_temp();
        v.extend(sl);
        v.push(Box::new(Statement::Move(
            Box::new(Expr::Temp(id)),
            Box::new(*el)
        )));
        v.extend(sr);
        let e = Expr::BinOp(
            Box::new(Expr::Temp(id)),
            op,
            Box::new(*er)
        );
        return (v, Box::new(e));
    }
    fn call(&mut self, l: Label, exprs: Vec<Box<Expr>>)
        -> (Vec<Box<Statement>>, Box<Expr>) {
        use Expr::*;
        let mut temps = Vec::<Box<Expr>>::new();
        let mut v = Vec::<Box<Statement>>::new();
        for e1 in exprs {
            let (s2, e2) = self.expression(e1);
            let id = self.create_temp();
            v.extend(s2);
            v.push(Box::new(Statement::Move(
                Box::new(Temp(id)), e2
            )));
            temps.push(Box::new(Temp(id)));
        }
        v.push(Box::new(Statement::Expr(
            Box::new(Call(l, temps))
        )));
        let id = self.create_temp();
        v.push(Box::new(Statement::Move(
            Box::new(Temp(id)),
            Box::new(Temp(self.reg.ret))
        )));
        return (v, Box::new(Temp(id)));
    }
    fn eseq(&mut self, s: Box<Statement>, e: Box<Expr>)
        -> (Vec<Box<Statement>>, Box<Expr>) {
        let mut s1 = self.statement(s);
        let (s2, e1) = self.expression(e);
        s1.extend(s2);
        return (s1, e1);
    }
    // Do when brain big
    #[allow(dead_code, unused_variables)]
    fn commute(l: Box<Expr>, r: Box<Expr>) -> bool { todo!() }
    fn create_temp(&mut self) -> u32 {
        self.reg.nids += 1;
        return self.reg.nids - 1;
    }
}