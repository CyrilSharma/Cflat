use crate::ir::{Statement, Expr, Operator};
use crate::aa::{AA, Reg};
pub struct Translator { 
    count: usize,
    costs: Vec<u32>,
    asm:   Vec<Vec<AA>>
}
impl Translator {
    pub fn new() -> Self { Self { count: 0, costs: vec![0, todo!()] } }
    pub fn translate(&mut self, stmts: &Vec<Statement>) -> Vec<AA> {
        let mut res = Vec::<AA>::new();
        for s in stmts {
            self.statement(s);
            res.extend(self.asm[self.count]);
        }
        return res;
    }
    fn statement(&mut self, s: &Statement) {
        let save = self.count;
        self.count += 1;
        use Statement::*;
        match s {
            Expr(e)        => self.expression(e),
            Move(d, s)     => self._move(d, s),
            Jump(j)        => self.jump(j),
            CJump(c, t, _) => self.cjump(c, *t),
            Label(l)  => vec![AA::Label(*l)],
            Return(r) => vec![AA::Ret],
            _ => unreachable!()
        }
    }
    fn expression(&mut self, e: &Expr) {
        if self.costs[self.count] != 0 { return }
        let save = self.count += 1;
        use Expr::*;
        use Operator::*;
        match e {
            Const(p) => self.add_label(&format!("Const ({:?})", p)),
            Temp(t) => self.add_label(&format!("Temp ({})", t)),
            UnOp(op, e) => self.unary(*op, e),
            BinOp(l, op, r) => self.binary(l, *op, r),
            Mem(e) => self.mem(e),
            Call(l, s) => self.call(*l, s),
            Address(e) => self.address(e),
            _ => unreachable!()
        }
    }
    fn _move(&mut self, d: &Expr, s: &Expr) {
        if self.costs[self.count] != 0 { return }
        use Expr::*;
        self.costs[self.count] = 1;
        self.asm[self.count] = match (d, s) {
            (Temp(a), Const(b))  => vec![AA::Mov1(Reg::ID(*a), b.bits())],
            (Temp(a), Temp(b))   => vec![AA::Mov2(Reg::ID(*a), Reg::ID(*b))],
            (Temp(a), Mem(T))    => {
                let Temp(i) = **T else { unreachable!() };
                vec![AA::LDR2(Reg::ID(*a), Reg::ID(i))]
            },
            (Mem(T), Const(b))   => {
                let Temp(i) = **T else { unreachable!() };
                vec![AA::STR2(Reg::ID(i), b.bits())]
            }
            (Mem(T), Temp(b))    => {
                let Temp(i) = **T else { unreachable!() };
                vec![AA::STR2(Reg::ID(i), Reg::ID(*b))]
            },
            (Mem(D), Mem(S)) => {
                self.costs[self.count] = 2;
                let id = self.create_temp();
                let Temp(r1) = **D else { unreachable!() };
                let Temp(r2) = **S else { unreachable!() };
                vec![
                    AA::LDR2(Reg::ID(id), Reg::ID(r2)),
                    AA::STR2(Reg::ID(r1), Reg::ID(id)),
                ]
            }
            _ => unreachable!()
        };
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
    fn eseq(&mut self, s: &Statement, e: &Expr) {
        let idx = self.count;
        self.add_label("ESEQ");
        self.add_edge(idx, self.count);
        self.statement(s);
        self.add_edge(idx, self.count);
        self.expression(e);
    }
    fn address(&mut self, e: &Expr) {
        let idx = self.count;
        self.add_label("Address");
        self.add_edge(idx, self.count);
        self.expression(e);
    }
    fn add_edge(&mut self, i: u32, j: u32) {
        println!("    node{} -> node{};", i, j)
    }
    fn add_label(&mut self, s: &str) {
        println!("{}", &format!(
            "    node{} [label=\"{}\"];",
            self.count, s
        ));
        self.count += 1;
    }
}