use crate::ast::*;
use crate::visitor::Visitor;
use crate::traverse::Traverseable;
use crate::symboltable::{SymbolTable, VSymbol, FSymbol};
struct Semantic {
    id_count: u32,
    fun_count: u32,
    func: FSymbol,
    vsym: SymbolTable<VSymbol>,
    fsym: SymbolTable<FSymbol>
}
impl Semantic {
    pub fn analyze(&mut self, m: &mut Module) {
        m.accept(self);
    }
    pub fn add_vsym(&mut self, s: &str, k: Kind) {
        self.vsym.insert(
            s, 
            VSymbol { 
                id: self.id_count,
                kind: k
            }
        );
        self.id_count += 1;
    }
}
impl Visitor for Semantic {
    fn handle_module(&mut self, m: &mut Module) {
        // Forward Declarations!
        for f in &m.functions {
            let args = f.params
                .iter()
                .map(|p| p.kind)
                .collect();
            self.fsym.insert(
                &f.name,
                FSymbol { 
                    id: self.fun_count, 
                    kind: f.ret, 
                    args
                }
            );
            self.fun_count += 1;
        }
    }
    fn begin_function_declaration(&mut self, f: &mut FunctionDeclaration) {
        self.vsym.scope_in();
        for p in &f.params {
            self.add_vsym(
                &p.name, 
                p.kind
            );
        }
        self.func = self.fsym.get(&f.name)
            .unwrap()
            .clone();
    }
    fn handle_function_declaration(&mut self, _f: &mut FunctionDeclaration) {
        self.vsym.scope_out();
    }
    fn handle_declare_statement(&mut self, d: &mut DeclareStatement) {
        if self.vsym.contains_key_in_scope(&d.name) {
            panic!("Defining already defined variable!");
        }
        self.add_vsym(&d.name, d.kind);
        if let Some(e) = &d.val {
            assert!(d.kind == e.kind.unwrap(), "{}",
                &format!(
                    "variable should have type {:?}, but is actually {:?}.",
                    d.kind, e.kind
                )
            )
        }
    }
    fn begin_for_statement(&mut self, _f: &mut ForStatement) {
        self.vsym.scope_in();
    }
    fn handle_for_statement(&mut self, _f: &mut ForStatement) {
        self.vsym.scope_out();
    }
    fn begin_compound_statement(&mut self, _c: &mut CompoundStatement) {
        self.vsym.scope_in();
    }
    fn handle_compound_statement(&mut self, _c: &mut CompoundStatement) {
        self.vsym.scope_out();
    }
    fn handle_jump_statement(&mut self, j: &mut JumpStatement) {
        if j.jump_type != JumpOp::Return { return; }
        match &j.expr {
            None => assert!(
                self.func.kind == Kind::void(),
                "Return mismatch"
            ),
            Some(e) => assert!(
                self.func.kind == e.kind.unwrap(),
                "Return mismatch"
            )
        }
    }
    fn handle_expr(&mut self, e: &mut Expr) {
        e.kind = match e.etype {
            ExprType::Access(ref a)     => a.kind,
            ExprType::Binary(ref b)     => b.kind,
            ExprType::Function(ref f)   => f.kind,
            ExprType::Unary(ref u)      => u.kind,
            ExprType::Identifier(ref i) => i.kind,
            ExprType::Integer(_)        => Some(Kind::int()),
            ExprType::Float(_)          => Some(Kind::float()) 
        }
    }
    fn handle_function_call(&mut self, f: &mut FunctionCall) {
        let fsym = match self.fsym.get(&f.name) {
            None => panic!("Reference To Non-Existing Function {}",
                        f.name),
            Some(name) => name
        };
        assert!(f.args.len() == fsym.args.len(),
            "Argument List does not match Function Argument List!"
        );
        for idx in 0..f.args.len() {
            let argk = f.args[idx].kind.unwrap();
            let fk = fsym.args[idx];
            if fk != argk {
                panic!("Argument Type Mismatch!")
            }
        }
    }
    fn handle_access(&mut self, _a: &mut AccessExpr) {
        panic!("THERE ARE NO ACCESSES...");
    }
    fn handle_unary(&mut self, u: &mut UnaryExpr) {
        let mut kind = u.expr.kind.unwrap();
        match u.unary_op {
            UnaryOp::Address => {
                if matches!(u.expr.etype, ExprType::Identifier(_)) {
                    kind.indir += 1;
                    u.kind = Some(kind);
                } else {
                    panic!("Cannot take Address of Non-Identifier!");
                }
            },
            UnaryOp::Star => {
                if matches!(u.expr.etype, ExprType::Identifier(_)) {
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
    fn handle_binary(&mut self, b: &mut BinaryExpr) {
        let lkind = b.left.kind.unwrap();
        let rkind = b.right.kind.unwrap();
        if lkind == rkind { 
            b.kind = Some(lkind); 
            return;
        }
        match b.binary_op {
            BinaryOp::Peq | BinaryOp::Seq |
            BinaryOp::Teq | BinaryOp::Deq |
            BinaryOp::Assign => {
                if lkind.prim == Primitive::Integer &&
                    rkind.prim == Primitive::Float {
                    panic!("Cannot assign Float to Int!");
                }
            }
            BinaryOp::Add | BinaryOp::Div |
            BinaryOp::Mul | BinaryOp::Sub => {
                if lkind.indir == 0 &&
                    rkind.indir == 0 {
                    b.kind = Some(Kind::float());
                } else {
                    panic!("Cannot add Integer and Float pointers!");
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
    fn handle_identifier(&mut self, i: &mut Identifier) {
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


