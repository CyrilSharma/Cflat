use crate::ast::*;
use crate::visitor::Visitor;
use crate::traverse::Traverseable;
use crate::symboltable::{SymbolTable, VSymbol, FSymbol};
struct Semantic {
    fname: String,
    vsym: SymbolTable<VSymbol>,
    fsym: SymbolTable<FSymbol>
}
// TODO: for proper error detection,
// Add errors to a list instead of Panicing.
// " Error Handling? Just don't make errors... "
impl Semantic {
    pub fn new() -> Self {
        Self {
            fname: String::new(),
            vsym: SymbolTable::new(),
            fsym: SymbolTable::new()
        }
    }
    pub fn analyze(&mut self, m: &mut Module) {
        // Forward Declarations!
        for f in &m.functions {
            self.fsym.insert(f);
        }
        m.accept(self);
    }
}
impl Visitor for Semantic {
    fn begin_function_declaration(&mut self, f: &mut FunctionDeclaration) {
        self.vsym.scope_in();
        for p in &f.params {
            self.vsym.insert(&p.name, p.kind);
        }
        self.fname = f.name.clone();
    }
    fn handle_function_declaration(&mut self, _f: &mut FunctionDeclaration) {
        self.vsym.scope_out();
    }
    fn handle_declare_statement(&mut self, d: &mut DeclareStatement) {
        if self.vsym.contains_key_in_scope(&d.name) {
            panic!("Defining already defined variable!");
        }
        self.vsym.insert(&d.name, d.kind);
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
        // println!("{}", self.fname);
        let func = self.fsym.get(&self.fname).unwrap();
        match &j.expr {
            None => assert!(
                func.kind == Kind::void(),
                "Return mismatch"
            ),
            Some(e) => assert!(
                func.kind == e.kind.unwrap(),
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
        };
        // println!("{:?}", e.kind);
    }
    fn handle_function_call(&mut self, f: &mut FunctionCall) {
        // println!("Call: {}", f.name);
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
        f.kind = Some(fsym.kind);
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


#[cfg(test)]
use std::fs;
use crate::parser::moduleParser;
use crate::printer::Printer;

#[test]
#[allow(dead_code)]
fn visualize() {
    // TODO: Reorganize test cases (only need a data directory)
    // Iterate over all test cases.
    let path0 = "tests/data/parser/input0.c";
    let input0 = fs::read_to_string(path0).expect("File not found!");
    let mut m = moduleParser::new().parse(&input0).expect("Parse Error!");
    Semantic::new().analyze(&mut m);
    Printer::new().print(&mut m);

    let path1 = "tests/data/parser/input1.c";
    let input1 = fs::read_to_string(path1).expect("File not found!");
    let mut m = moduleParser::new().parse(&input1).expect("Parse Error!");
    Semantic::new().analyze(&mut m);
    Printer::new().print(&mut m);

    let path1 = "tests/data/parser/input2.c";
    let input1 = fs::read_to_string(path1).expect("File not found!");
    let mut m = moduleParser::new().parse(&input1).expect("Parse Error!");
    Semantic::new().analyze(&mut m);
    Printer::new().print(&mut m);
}