use crate::ast::*;
use crate::visitor::Visitor;
pub trait Traverseable {
    fn accept<T: Visitor>(&mut self, v: &mut T);
}

impl Traverseable for Module {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.handle_module(self);
        for f in &mut self.functions { f.accept(v); }
        v.cleanup();
    }
}

impl Traverseable for FunctionDeclaration {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.handle_function_declaration(self);
        self.statement.accept(v);
        v.cleanup();
    }
}

impl Traverseable for Statement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.handle_statement(self);
        match self {
            Statement::Declare(d)  => d.accept(v),
            Statement::Expr(e)     => e.accept(v),
            Statement::If(i)       => i.accept(v),
            Statement::For(f)      => f.accept(v),
            Statement::While(w)    => w.accept(v),
            Statement::Compound(c) => c.accept(v),
            Statement::Jump(j)     => j.accept(v)
        }
        v.cleanup();
    }
}

impl Traverseable for DeclareStatement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.handle_declare_statement(self);
        if let Some(e) = &mut self.val { e.accept(v); }
        v.cleanup();
    }
}

impl Traverseable for ExprStatement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.handle_expr_statement(self);
        if let Some(e) = &mut self.expr { e.accept(v); }
        v.cleanup();
    }
}

impl Traverseable for IfStatement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.handle_if_statement(self);
        self.condition.accept(v);
        self.true_stmt.accept(v);
        if let Some(e) = &mut self.false_stmt { e.accept(v); }
        v.cleanup();
    }
}

impl Traverseable for ForStatement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.handle_for_statement(self);
        if let Some(e) = &mut self.init { e.accept(v); }
        if let Some(e) = &mut self.each { e.accept(v); }
        if let Some(e) = &mut self.end  { e.accept(v); }
        self.stmt.accept(v);
        v.cleanup();
    }
}

impl Traverseable for WhileStatement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.handle_while_statement(self);
        self.condition.accept(v);
        self.stmt.accept(v);
        v.cleanup();
    }
}

impl Traverseable for CompoundStatement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.handle_compound_statement(self);
        if let Some(vec) = &mut self.stmts {
            for s in vec { s.accept(v); }
        }
        v.cleanup();
    }
}

impl Traverseable for JumpStatement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.handle_jump_statement(self);
        if let Some(e) = &mut self.expr { e.accept(v) }
        v.cleanup();
    }
}

impl Traverseable for Expr {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.handle_expr(self);
        match self {
            Expr::Function(f)   => f.accept(v),
            Expr::Access(a)     => a.accept(v),
            Expr::Unary(u)      => u.accept(v),
            Expr::Binary(b)     => b.accept(v),
            Expr::Integer(_i)    => (),
            Expr::Float(_f)      => (),
            Expr::Identifier(_i) => (),
        }
        v.cleanup();
    }
}

impl Traverseable for FunctionCall {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.handle_function_call(self);
        if let Some(vec) = &mut self.args {
            for e in vec { e.accept(v); }
        }
        v.cleanup();
    }
}

impl Traverseable for AccessExpr {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.handle_access(self);
        self.offset.accept(v);
        v.cleanup();
    }
}

impl Traverseable for UnaryExpr {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.handle_unary(self);
        self.expr.accept(v);
        v.cleanup();
    }
}

impl Traverseable for BinaryExpr {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.handle_binary(self);
        self.left.accept(v);
        self.right.accept(v);
        v.cleanup();
    }
}