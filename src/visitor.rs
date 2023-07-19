use crate::ast::*;
#[allow(unused_variables)]
pub trait Visitor {
    fn handle_module(&mut self, m: &mut Module) {}
    fn begin_function_declaration(&mut self, f: &mut FunctionDeclaration) {} // Need special handling.
    fn handle_function_declaration(&mut self, f: &mut FunctionDeclaration) {}
    fn handle_statement(&mut self, s: &mut Statement) {}
    fn handle_declare_statement(&mut self, d: &mut DeclareStatement) {}
    fn handle_expr_statement(&mut self, e: &mut ExprStatement) {}
    fn handle_if_statement(&mut self, i: &mut IfStatement) {}
    fn begin_for_statement(&mut self, f: &mut ForStatement) {}
    fn handle_for_statement(&mut self, f: &mut ForStatement) {}
    fn handle_while_statement(&mut self, w: &mut WhileStatement) {}
    fn begin_compound_statement(&mut self, c: &mut CompoundStatement) {}
    fn handle_compound_statement(&mut self, c: &mut CompoundStatement) {}
    fn handle_jump_statement(&mut self, j: &mut JumpStatement) {}
    fn handle_expr(&mut self, e: &mut Expr) {}
    fn handle_function_call(&mut self, f: &mut FunctionCall) {}
    fn handle_access(&mut self, a: &mut AccessExpr) {}
    fn handle_unary(&mut self, u: &mut UnaryExpr) {}
    fn handle_binary(&mut self, b: &mut BinaryExpr) {}
    fn handle_integer(&mut self, i: i32) {}
    fn handle_float(&mut self, f: f32) {}
    fn handle_identifier(&mut self, i: &Identifier) {}
    fn setup(&mut self) {}
}