use crate::ir::*;
pub struct Reducer { count: u32, ret: u32 }
impl Reducer {
    pub fn new(nid: u32) -> Self { Self{count: nid+1, ret: nid } }
    pub fn reduce(&mut self, stmts: &[Statement]) -> Vec<Statement> {
        return self.seq(stmts);
    }
    fn statement(&mut self, s: &Statement) -> Vec<Statement> {
        use Statement::*;
        match s {
            Expr(e) => self.expr_statement(e),
            CJump(e, t, f) => {
                let (mut s1, e1) = self.expression(e);
                s1.push(CJump(e1, *t, *f));
                return s1;
            },
            Jump(_) | Label(_) => return vec![s.clone()],
            Return(r)  => match r {
                None => return vec![Return(None)],
                Some(e) => {
                    let (mut s1, e1) = self.expression(e);
                    s1.push(Return(Some(e1)));
                    return s1;
                }
            }
            Move(d, s) => return self._move(d, s),
            Seq(s) => return self.seq(s),
        }
    }
    fn expr_statement(&mut self, e: &Expr) -> Vec<Statement> {
        let (mut s1, e1) = self.expression(e);
        match *e1 {
            Expr::Call(_, _) => s1.push(Statement::Expr(e1)),
            _ => ()
        }
        return s1;
    }
    fn _move(&mut self, d: &Expr, s: &Expr) -> Vec<Statement> {
        use Expr::*;
        match d {
            Temp(_) => {
                let (mut s1, e1) = self.expression(s);
                s1.push(Statement::Move(
                    Box::new(d.clone()),
                    e1
                ));
                return s1;
            }
            Mem(_) => {
                /* TODO: check e1 & e2 commute? */
                let id = self.create_temp();
                let mut v = Vec::<Statement>::new();
                let (sl, el) = self.expression(d);
                let (sr, er) = self.expression(s);
                v.extend(sl);
                v.push(Statement::Move(
                    Box::new(Expr::Temp(id)),
                    el,
                ));
                v.extend(sr);
                v.push(Statement::Move(
                    Box::new(Temp(id)),
                    er
                ));
                return v;
            }
            _ => unreachable!()
        }
    }
    fn seq(&mut self, stmts: &[Statement]) -> Vec<Statement> {
        let mut v = Vec::<Statement>::new();
        for s in stmts {
            v.extend(self.statement(s));
        }
        return v;
    }
    fn expression(&mut self, e: &Expr) -> (Vec<Statement>, Box<Expr>) {
        use Expr::*;
        match e {
            Const(_) | Temp(_) => {
                return (Vec::new(), Box::new(e.clone()))
            }
            Mem(e1) | Address(e1) => {
                let (v, e2) = self.expression(e1);
                return (v, Box::new(Mem(e2)))
            }
            UnOp(op, e) => return self.unary(*op, e),
            BinOp(l, op, r) => return self.binary(l, *op, r),
            Call(l, exprs) => return self.call(*l, exprs),
            ESeq(s, e) => return self.eseq(s, e)
        }
    }
    fn unary(&mut self, op: Operator, e: &Expr) -> (Vec<Statement>, Box<Expr>) {
        let (s1, e1) = self.expression(e);
        return (s1, Box::new(Expr::UnOp(op, Box::new(*e1))));
    }
    fn binary(&mut self, l: &Expr, op: Operator, r: &Expr) -> (Vec<Statement>, Box<Expr>) {
        /* This can be made more efficient if you can somehow determine if the operators commute */
        let mut v = Vec::<Statement>::new();
        let (sl, el) = self.expression(l);
        let (sr, er) = self.expression(r);
        let id = self.create_temp();
        v.extend(sl);
        v.push(Statement::Move(
            Box::new(Expr::Temp(id)),
            Box::new(*el)
        ));
        v.extend(sr);
        let e = Expr::BinOp(
            Box::new(Expr::Temp(id)),
            op,
            Box::new(*er)
        );
        return (v, Box::new(e));
    }
    fn call(&mut self, l: Label, exprs: &[Expr]) -> (Vec<Statement>, Box<Expr>) {
        use Expr::*;
        let mut temps = Vec::<Expr>::new();
        let mut v = Vec::<Statement>::new();
        for e1 in exprs {
            let (s2, e2) = self.expression(e1);
            let t = Temp(self.create_temp());
            v.extend(s2);
            v.push(Statement::Move(
                Box::new(t.clone()),
                Box::new(*e2)
            ));
            temps.push(t);
        }
        v.push(Statement::Expr(
            Box::new(Call(l, temps))
        ));
        let id = self.create_temp();
        v.push(Statement::Move(
            Box::new(Temp(id)),
            Box::new(Temp(self.ret))
        ));
        return (v, Box::new(Temp(id)));
    }
    fn eseq(&mut self, s: &Statement, e: &Expr) -> (Vec<Statement>, Box<Expr>) {
        let mut s1 = self.statement(s);
        let (s2, e1) = self.expression(e);
        s1.extend(s2);
        return (s1, e1);
    }
    // Do when brain big
    #[allow(dead_code, unused_variables)]
    fn commute(l: &Expr, r: &Expr) -> bool { todo!() }
    fn create_temp(&mut self) -> u32 {
        self.count += 1;
        return self.count - 1;
    }
}