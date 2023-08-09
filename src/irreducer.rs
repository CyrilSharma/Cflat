use crate::ir::*;
use bumpalo::Bump;
pub struct Reducer {
    arena: Bump,
    count: u32,
    ret: u32
}
impl Reducer {
    pub fn new(nid: u32) -> Self { 
        Self {
            arena: Bump::new(),
            count: nid+1,
            ret: nid
        }
    }
    pub fn reduce(&mut self, stmts: &Vec<Statement>) -> Vec<Statement> {
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
                    self.arena.alloc(d.clone()),
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
                    self.arena.alloc(Expr::Temp(id)),
                    el,
                ));
                v.extend(sr);
                v.push(Statement::Move(
                    self.arena.alloc(Temp(id)),
                    er
                ));
                return v;
            }
            _ => unreachable!()
        }
    }
    fn seq(&mut self, stmts: &Vec<Statement>) -> Vec<Statement> {
        let mut v = Vec::<Statement>::new();
        for s in stmts {
            v.extend(self.statement(s));
        }
        return v;
    }
    fn expression(&mut self, e: &Expr) -> (Vec<Statement>, &Expr) {
        use Expr::*;
        match e {
            Mem(e1) | Address(e1) => {
                let (v, e2) = self.expression(e1);
                return (v, self.arena.alloc(Mem(e2)))
            },
            UnOp(op, e) => return self.unary(*op, e),
            BinOp(l, op, r) => return self.binary(l, *op, r),
            Call(l, exprs) => return self.call(*l, exprs),
            ESeq(s, e) => return self.eseq(s, e),
            Const(_) | Temp(_) => return (Vec::new(), e)
        }
    }
    fn unary(&mut self, op: Operator, e: &Expr) -> (Vec<Statement>, &Expr) {
        let (s1, e1) = self.expression(e);
        let e = self.arena.alloc(
            Expr::UnOp(op, self.arena.alloc(*e1))
        );
        return (s1, e);
    }
    fn binary(&mut self, l: &Expr, op: Operator, r: &Expr) -> (Vec<Statement>, &Expr) {
        /* This can be made more efficient if you can somehow determine if the operators commute */
        let mut v = Vec::<Statement>::new();
        let (sl, el) = self.expression(l);
        let (sr, er) = self.expression(r);
        let id = self.create_temp();
        v.extend(sl);
        v.push(Statement::Move(
            self.arena.alloc(Expr::Temp(id)),
            self.arena.alloc(*el)
        ));
        v.extend(sr);
        let e = Expr::BinOp(
            self.arena.alloc(Expr::Temp(id)),
            op,
            self.arena.alloc(*er)
        );
        return (v, self.arena.alloc(e));
    }
    fn call(&mut self, l: Label, exprs: &Vec<Expr>) -> (Vec<Statement>, &Expr) {
        use Expr::*;
        let mut temps = Vec::<Expr>::new();
        let mut v = Vec::<Statement>::new();
        for e1 in exprs {
            let (s2, e2) = self.expression(e1);
            v.extend(s2);
            let i = self.create_temp();
            v.push(Statement::Move(
                self.arena.alloc(Temp(i)),
                self.arena.alloc(*e2)
            ));
            temps.push(Temp(i));
        }
        v.push(Statement::Expr(
            self.arena.alloc(Call(l, temps))
        ));
        let id = self.create_temp();
        v.push(Statement::Move(
            self.arena.alloc(Temp(id)),
            self.arena.alloc(Temp(self.ret))
        ));
        return (v, self.arena.alloc(Temp(id)));
    }
    fn eseq(&mut self, s: &Statement, e: &Expr) -> (Vec<Statement>, &Expr) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use crate::parser::moduleParser;
    use crate::semantic::Semantic;
    use crate::astprinter;
    use crate::irprinter;
    use crate::irtranslator::Translator;

    #[test]
    fn visualize() {
        let mut i = 0;
        let dir = "tests/data/";
        while Path::new(&format!("{dir}/input{i}.c")).exists() {
            let filepath = &format!("{dir}/input{i}.c");
            let input = fs::read_to_string(filepath).expect("File not found!");
            let mut m = moduleParser::new().parse(&input).expect("Parse Error!");
            let mut semantic = Semantic::new();
            semantic.analyze(&mut m);
            let ir  = Translator::new().translate(&mut m);
            let lir = Reducer::new(semantic.nid()).reduce(&ir);
            astprinter::Printer::new().print(&m);
            irprinter::Printer::new().print(&lir);
            i += 1;
        }
    }
}