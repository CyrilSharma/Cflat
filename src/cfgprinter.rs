use crate::ir::*;
use crate::cfg::*;
pub struct Printer { count: u32 }
/* note that this only works for LIR CFG */
impl Printer {
    pub fn new() -> Self { Self{count: 0} }
    pub fn print(&mut self, cfg: &CFG) {
        println!("digraph CFG {{");
        println!(r#"  node [shape=box, fontname="Helvetica", fontsize=12]"#);
        for i in 0..cfg.starts.len() {
            let idx = cfg.starts[i];
            println!("{}", format!(
                r#"  node{} [label="Start {}"]"#,
                cfg.nodes.len() + i, idx as u32
            ));
            self.add_edge(
                (cfg.nodes.len() + i) as u32, 
                idx as u32
            );
        }
        for n in &cfg.nodes {
            self.node(n);
            self.count += 1;
        }
        println!("}}");
    }
    fn node(&mut self, n: &Node) {
        let strs = n.stmts.iter()
            .map(|x| self.statement(x))
            .collect();
        self.label(strs);
        if let Some(e) = n.t { 
            self.add_edge(self.count, e as u32) 
        }
        if let Some(e) = n.f { 
            self.add_edge(self.count, e as u32) 
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
            _ => unreachable!()
        };
    }
    fn _move(&mut self, d: &Expr, s: &Expr) -> String {
        return format!("Move {} {}",
            self.expression(d), self.expression(s)
        );
    }
    fn cjump(&mut self, j: &Expr, t: Label, f: Label) -> String {
        return format!("CJump {} {} {}",
            self.expression(j),
            format!("{}", t),
            format!("{}", f),
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
            _ => unreachable!()
        }
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
        return format!("Addr({})", self.expression(e));
    }
    fn add_edge(&mut self, i: u32, j: u32) {
        println!("    node{} -> node{};", i, j)
    }
    fn label(&self, s: Vec<String>) {
        let stmts = s.iter() 
            .map(|x| format!(
                "          <tr><td align=\"center\">{}</td></tr>\n",
                x
            ))
            .collect::<Vec<String>>()
            .join("");
        println!("{}",
            format!( "  node{} [label=<\n", self.count) +
            &format!("    <table border=\"0\" cellborder=\"1\" cellspacing=\"0\" cellpadding=\"6\">\n") + 
            &format!("      <tr><td align=\"center\" bgcolor=\"lightblue\"><font color=\"black\"><b>Node {}</b></font></td></tr>\n", self.count) + 
            
            &format!("{}", stmts) + 
            
            &format!("    </table>\n") + 
            &format!("  >]") 
        );
    }
}