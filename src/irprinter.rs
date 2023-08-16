use crate::ir::*;
pub struct Printer { tabs: usize }
/* note that this only works for LIR CFG */
impl Printer {
    pub fn new() -> Self { Self{tabs: 0} }
    pub fn print(&mut self, stmts: &[Box<Statement>]) {
        for s in stmts {
            println!("{}\n",
                self.statement(s)
            );
        }
    }
    fn statement(&mut self, s: &Statement) -> String {
        use Statement::*;
        return match s {
            Expr(e) => self.expression(e),
            Move(d, s) => self._move(d, s),
            Jump(j) => format!("Jump {:?}", j),
            CJump(c, t, f) => self.cjump(c, *t, *f),
            Label(l) => format!("{}: ", l),
            Return(r) => self._return(r),
            Seq(s) => self.seq(s)
        };
    }
    fn seq(&mut self, stmts: &[Box<Statement>]) -> String {
        let tabs = "  ".repeat(self.tabs).to_string();
        self.tabs += 1;
        let mut res = "Seq(\n".to_string();
        for s in stmts {
            res.push_str(&format!("{}{},\n",
                "  ".repeat(self.tabs),
                self.statement(s)
            ));
        }
        res.push_str(&(tabs.clone() + ")"));
        self.tabs -= 1;
        return res;
    }
    fn _move(&mut self, d: &Expr, s: &Expr) -> String {
        return format!("Move {} {}",
            self.expression(d), self.expression(s)
        );
    }
    fn cjump(&mut self, j: &Expr, t: Label, f: Label) -> String {
        return format!("CJump {} {} {}",
            self.expression(j),
            format!("{:?}", t),
            format!("{:?}", f),
        );
    }
    fn _return(&mut self, r: &Option<Box<Expr>>) -> String {
        return format!("Return {}",
            match r {
                None => format!(""),
                Some(e) => format!("{}",
                    self.expression(e)
                )
            }
        );
    } 
    fn expression(&mut self, e: &Expr) -> String {
        use Expr::*;
        return match e {
            Const(p) => format!("{:?}", p),
            Temp(t) => format!("T({})", t),
            UnOp(op, e) => self.unary(*op, e),
            BinOp(l, op, r) => self.binary(l, *op, r),
            Mem(e) => self.mem(e),
            Call(l, s) => self.call(*l, s),
            Address(e) => self.address(e),
            ESeq(s, e) => self.eseq(s, e)
        }
    }
    fn eseq(&mut self, s: &Statement, e: &Expr) -> String {
        return format!("ESeq({}, #{}#)",
            self.statement(s),
            self.expression(e)
        );
    }
    fn unary(&mut self, op: Operator, e: &Expr) -> String {
        return format!("{:?} {}", op, self.expression(e));
    }
    fn binary(&mut self, l: &Expr, op: Operator, r: &Expr) -> String {
        return format!("{:?} {} {}", 
            op,
            self.expression(l),
            self.expression(r)
        );
    }
    fn mem(&mut self, m: &Expr) -> String {
        return format!("Mem({})",
            self.expression(m)
        );
    }
    fn call(&mut self, l: Label, v: &[Box<Expr>]) -> String {
        return format!("Call(f={}, {})", l,
            v.iter().map(|e| self.expression(e))
                .collect::<Vec<String>>().join(", ")
        );
    }
    fn address(&mut self, e: &Expr) -> String {
        return format!("&{}", self.expression(e));
    }
}