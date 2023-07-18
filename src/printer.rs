use crate::ast::*;
use crate::traverse::Traverseable;
use crate::visitor::Visitor;
// Re-implement with Visitor pattern.
pub struct Printer { 
    count: u32,
    stk: Vec<u32>
}
impl Printer {
    pub fn new() -> Self { 
        Self {
            count: 0,
            stk: Vec::new()
        } 
    }
    pub fn print(&mut self, m: &mut Module) {
        println!("digraph AST {{");
        m.accept(self);
        println!("}}");
    }
    fn make_node(&mut self, s: &str) {
        let c = self.stk.pop().unwrap();
        println!("{}", &format!(
            "    node{} [label=\"{}\"];",
            c, s
        ));
        if let Some(p) = self.stk.last() {
            println!("    node{} -> node{};",
                p, c);
        }
    }
}
#[allow(unused_variables)]
impl Visitor for Printer {
    fn handle_module(&mut self, m: &mut Module) {
        self.make_node("Module");
    }
    fn handle_function_declaration(&mut self, f: &mut FunctionDeclaration) {
        let mut kind_str = format!("{:?}", f.ret.prim);
        for _ in 0..f.ret.indirection { kind_str.push('*'); }
        self.make_node(&format!("Declare {kind_str} {}()", f.name));
    }
    fn handle_declare_statement(&mut self, d: &mut DeclareStatement) {
        let mut kind_str = format!("{:?}", d.kind.prim);
        for _ in 0..d.kind.indirection { kind_str.push('*'); }
        self.make_node(&format!("Declare: {kind_str} {}", d.name));
    }
    fn handle_expr_statement(&mut self, e: &mut ExprStatement) {
        self.make_node("Expression Statement");
    }
    fn handle_if_statement(&mut self, i: &mut IfStatement) {
        self.make_node("If");
    }
    fn handle_for_statement(&mut self, f: &mut ForStatement) {
        self.make_node("For");
    }
    fn handle_while_statement(&mut self, w: &mut WhileStatement) {
        self.make_node("While");
    }
    fn handle_compound_statement(&mut self, c: &mut CompoundStatement) {
        self.make_node("Compound Statement");
    }
    fn handle_jump_statement(&mut self, j: &mut JumpStatement) {
        self.make_node("Jump Statement");
    }
    fn handle_expr(&mut self, e: &mut Expr) {
        match e {
            Expr::Integer(i)    => self.handle_integer(*i),
            Expr::Float(f)      => self.handle_float(*f),
            Expr::Identifier(i) => self.handle_identifier(i),
            _ => ()
        }
    }
    fn handle_function_call(&mut self, f: &mut FunctionCall) {
        self.make_node(&format!("Call Function: {}", f.name));
    }
    fn handle_access(&mut self, a: &mut AccessExpr) {
        self.make_node(&format!("Access: {}", a.name));
    }
    fn handle_unary(&mut self, u: &mut UnaryExpr) {
        self.make_node(&format!("Unary: {:?}", u.unary_op));
    }
    fn handle_binary(&mut self, b: &mut BinaryExpr) {
        self.make_node(&format!("Binary: {:?}", b.binary_op));
    }
    fn handle_integer(&mut self, i: i32) {
        self.setup();
        self.make_node(&format!("Integer: {}", i));
    }
    fn handle_float(&mut self, f: f32) {
        self.setup();
        self.make_node(&format!("Float: {}", f));
    }
    fn handle_identifier(&mut self, s: &str) {
        self.setup();
        self.make_node(&format!("Identifier: {}", s));
    }
    fn setup(&mut self) { 
        self.stk.push(self.count);
        self.count += 1;
    }
}