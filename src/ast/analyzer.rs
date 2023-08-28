use super::ast::*;
use crate::registry::Registry;
use super::symboltable::{SymbolTable, VSymbol, FSymbol};
pub struct Analyzer<'l> {
    fname: String,
    vsym: SymbolTable<VSymbol>,
    fsym: SymbolTable<FSymbol>,
    reg:  &'l mut Registry
}
// TODO: for proper error detection,
// Add errors to a list instead of Panicing.
// " Error Handling? Just don't make errors... "
impl<'l> Analyzer<'l> {
    pub fn new(registry: &'l mut Registry) -> Self {
        Self {
            fname: String::new(),
            vsym: SymbolTable::new(),
            fsym: SymbolTable::new(),
            reg:  registry
        }
    }
    pub fn analyze(&mut self, m: &mut Module) {
        // Forward Declarations!
        for i in 0..m.functions.len() {
            if m.functions[i].name != "main" { continue; }
            m.functions[i].id = self.fsym
                .insert(&m.functions[i]);
        }
        for i in 0..m.functions.len() {
            if m.functions[i].name == "main" { continue; }
            m.functions[i].id = self.fsym
                .insert(&m.functions[i]);
        }
        for f in &mut m.functions {
            self.function_declaration(f);
        }
        self.reg.nids = self.vsym.count;
    }
    fn function_declaration(&mut self, f: &mut FunctionDeclaration) {
        self.vsym.scope_in();
        for p in &mut f.params {
            p.id = self.vsym.insert(&p.name, p.kind);
        }
        self.fname = f.name.clone();
        self.statement(&mut f.stmt);
        self.vsym.scope_out();
    }
    fn statement(&mut self, s: &mut Statement) {
        use Statement::*;
        match s {
            Declare     (ref mut d) => self.declare_statement(d),
            Expr        (ref mut e) => self.expr_statement(e),
            If          (ref mut i) => self.if_statement(i),
            For         (ref mut f) => self.for_statement(f),
            While       (ref mut w) => self.while_statement(w),
            Compound    (ref mut c) => self.compound_statement(c),
            Jump        (ref mut j) => self.jump_statement(j),
        }
    }
    fn declare_statement(&mut self, d: &mut DeclareStatement) {
        if self.vsym.contains_key_in_scope(&d.name) {
            panic!("Defining already defined variable!");
        }
        d.id = self.vsym.insert(&d.name, d.kind);
        if let Some(e) = &mut d.val {
            self.expression(e);
            assert!(d.kind == e.kind().unwrap(), "{}",
                &format!(
                    "variable should have type {:?}, but is actually {:?}.",
                    d.kind, e.kind()
                )
            )
        }
    }
    fn if_statement(&mut self, i: &mut IfStatement) {
        self.expression(&mut i.condition);
        self.statement(&mut i.true_stmt);
        if let Some(s) = &mut i.false_stmt {
            self.statement(s);
        }
    }
    fn expr_statement(&mut self, e: &mut ExprStatement) {
        if let Some(e) = &mut e.expr {
            self.expression(e);
        }
    }
    fn for_statement(&mut self, f: &mut ForStatement) {
        self.vsym.scope_in();
        self.statement(&mut f.init);
        if let Some(e) = &mut f.cond {
            self.expression(e);
        }
        if let Some(e) = &mut f.each {
            self.expression(e);
        }
        self.statement(&mut f.stmt);
        self.vsym.scope_out();
    }
    fn while_statement(&mut self, w: &mut WhileStatement) {
        self.vsym.scope_in();
        self.expression(&mut w.condition);
        self.statement(&mut w.stmt);
        self.vsym.scope_out();
    }
    fn compound_statement(&mut self, c: &mut CompoundStatement) {
        self.vsym.scope_in();
        for mut stmt in &mut c.stmts {
            self.statement(&mut stmt);
        }
        self.vsym.scope_out();
    }
    fn jump_statement(&mut self, j: &mut JumpStatement) {
        if j.jump_type != JumpOp::Return { return; }
        let kind = match &mut j.expr {
            None => Some(Kind::void()),
            Some(e) => {
                self.expression(e);
                e.kind()
            }
        };
        let func = self.fsym.get(&self.fname).unwrap();
        assert!(func.kind == kind.unwrap());
    }
    fn function_call(&mut self, f: &mut FunctionCall) {
        for e in &mut f.args { self.expression(e); }
        let fsym = match self.fsym.get(&f.name) {
            None => panic!("Reference To Non-Existing Function {}",
                        f.name),
            Some(name) => name
        };
        assert!(f.args.len() == fsym.args.len(),
            "Argument List does not match Function Argument List!"
        );
        for idx in 0..f.args.len() {
            let argk = f.args[idx].kind().unwrap();
            let fk = fsym.args[idx];
            if fk != argk {
                panic!("Argument Type Mismatch!")
            }
        }
        f.kind = Some(fsym.kind);
        f.id = fsym.id;
    }
    fn expression(&mut self, e: &mut Expr) {
        use Expr::*;
        match e {
            Function (ref mut i) => self.function_call(i),
            Access   (ref mut i) => self.access(i),
            Unary    (ref mut i) => self.unary(i),
            Binary   (ref mut i) => self.binary(i),
            Ident    (ref mut i) => self.identifier(i),
            _                    => ()
        }
    }
    fn access(&mut self, _a: &mut AccessExpr) {
        panic!("THERE ARE NO ACCESSES...");
    }
    fn unary(&mut self, u: &mut UnaryExpr) {
        self.expression(&mut u.expr);
        let mut kind = u.expr.kind().unwrap();
        match u.unary_op {
            UnaryOp::Address => {
                if matches!(*u.expr, Expr::Ident(_)) {
                    kind.indir += 1;
                    u.kind = Some(kind);
                } else {
                    panic!("Cannot take Address of Non-Identifier!");
                }
            },
            UnaryOp::Star => {
                if matches!(*u.expr, Expr::Ident(_)) {
                    if kind.indir == 0 {
                        panic!("Cannot dereferencea Primitive!");
                    } else {
                        kind.indir -= 1;
                    }
                    u.kind = Some(kind);
                } else {
                    panic!("Cannot dereference Non-Identifier!");
                }
            },
            UnaryOp::Neg => {
                if kind.indir == 0 {
                    u.kind = Some(kind);
                } else {
                    panic!("Cannot Negate Address!");
                }
            },
            UnaryOp::Not => {
                if kind != Kind::int() {
                    u.kind = Some(kind);
                } else {
                    panic!("Cannot only NOT Integers!");
                }
            },
        }
    }
    fn binary(&mut self, b: &mut BinaryExpr) {
        self.expression(&mut b.left);
        self.expression(&mut b.right);
        let lkind = b.left.kind().unwrap();
        let rkind = b.right.kind().unwrap();
        if lkind == rkind { 
            b.kind = Some(lkind); 
            return;
        }
        match b.binary_op {
            BinaryOp::Peq | BinaryOp::Seq |
            BinaryOp::Teq | BinaryOp::Deq |
            BinaryOp::Assign => {
                if lkind == Kind::float() &&
                    rkind == Kind::int() {
                    b.kind = Some(lkind);
                    return;
                }
                panic!("Invalid Assignment!");
            }
            BinaryOp::Add | BinaryOp::Div |
            BinaryOp::Mul | BinaryOp::Sub => {
                if lkind == Kind::float() ||
                    rkind == Kind::float() {
                    b.kind = Some(Kind::float());
                } else if lkind.indir != 0 ||
                    rkind.indir != 0 {
                    if lkind == Kind::int() {
                        b.kind = Some(rkind);
                    } else if rkind == Kind::int() {
                        b.kind = Some(lkind);
                    } else {
                        panic!("Cannot apply Binary Op to two pointers!");
                    }
                } else {
                    b.kind = Some(Kind::int());
                }
            },
            BinaryOp::Eq | BinaryOp::Geq |
            BinaryOp::Gt | BinaryOp::Leq |
            BinaryOp::Neq | BinaryOp::Lt => {
                if lkind.indir == 0 &&
                    rkind.indir == 0 {
                    b.kind = Some(Kind::int());
                } else {
                    panic!("Cannot compare two pointers!");
                }
            },
            BinaryOp::Or | BinaryOp::And => {
                if lkind == Kind::int() &&
                    rkind == Kind::float() {
                    b.kind = Some(Kind::int());
                } else {
                    panic!("Can only OR or AND two Integers!");
                }
            }
        }
    }
    fn identifier(&mut self, i: &mut Identifier) {
        match self.vsym.get(&i.name) {
            None => panic!(
                "Identifier {} not found!",
                i.name
            ),
            Some(s) => {
                i.kind = Some(s.kind);
                i.id = s.id;
            }
        }
    }
}