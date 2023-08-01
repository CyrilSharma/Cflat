use crate::ir::*;
pub struct Printer { count: u32 }
impl Printer {
    pub fn new() -> Self { Self{count: 0} }
    pub fn print(&mut self, stmts: &Vec<Statement>) {
        println!("digraph IR {{");
        for s in stmts {
            self.statement(s);
        }
        println!("}}");
    }
    fn statement(&mut self, s: &Statement) {
        use Statement::*;
        match s {
            Expr(e) => self.expression(e),
            Move(d, s) => self._move(d, s),
            Seq(s) => self.seq(s),
            Jump(j) => self.add_label(&format!("Jump: {:?}", j)),
            CJump(c, t, f) => self.cjump(c, *t, *f),
            Label(l) => self.add_label(&format!("{}", l)),
            Return(r) => self._return(r)
        }
    }
    fn _move(&mut self, d: &Expr, s: &Expr) {
        let idx = self.count;
        self.add_label("Move");
        self.add_edge(idx, self.count);
        self.expression(d);
        self.add_edge(idx, self.count);
        self.expression(s);
    }
    fn seq(&mut self, stmts: &Vec<Statement>) {
        let idx = self.count;
        self.add_label("Seq");
        for s in stmts {
            self.add_edge(idx, self.count);
            self.statement(s);
        }
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
    fn _return(&mut self, r: &Option<Box<Expr>>) {
        match *r {
            None => self.add_label("Return"),
            Some(ref e) => {
                let idx = self.count;
                self.add_label("Return");
                self.add_edge(idx, self.count);
                self.expression(&e);
            }
        }
    } 
    fn expression(&mut self, e: &Expr) {
        use Expr::*;
        match e {
            Const(p) => self.add_label(&format!("Const ({:?})", p)),
            Temp(t) => self.add_label(&format!("Temp ({})", t)),
            UnOp(op, e) => self.unary(*op, e),
            BinOp(l, op, r) => self.binary(l, *op, r),
            Mem(e) => self.mem(e),
            Call(l, s) => self.call(*l, s),
            ESeq(s, e) => self.eseq(s, e),
            Address(e) => self.address(e)
        }
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