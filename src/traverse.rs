use crate::ast::*;
use crate::visitor::Visitor;
pub trait Traverseable {
    fn accept<T: Visitor>(&mut self, v: &mut T);
}

impl Traverseable for Module {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.setup();
        for f in &mut self.functions { f.accept(v); }
        v.handle_module(self);
    }
}

impl Traverseable for FunctionDeclaration {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.setup();
        v.begin_function_declaration(self);
        self.statement.accept(v);
        v.handle_function_declaration(self);
    }
}

impl Traverseable for Statement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        match self {
            Statement::Declare(d)  => d.accept(v),
            Statement::Expr(e)     => e.accept(v),
            Statement::If(i)       => i.accept(v),
            Statement::For(f)      => f.accept(v),
            Statement::While(w)    => w.accept(v),
            Statement::Compound(c) => c.accept(v),
            Statement::Jump(j)     => j.accept(v)
        }
        v.handle_statement(self);
    }
}

impl Traverseable for DeclareStatement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.setup();
        if let Some(e) = &mut self.val { e.accept(v); }
        v.handle_declare_statement(self);
    }
}

impl Traverseable for ExprStatement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.setup();
        if let Some(e) = &mut self.expr { e.accept(v); }
        v.handle_expr_statement(self);
    }
}

impl Traverseable for IfStatement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.setup();
        self.condition.accept(v);
        self.true_stmt.accept(v);
        if let Some(e) = &mut self.false_stmt { e.accept(v); }
        v.handle_if_statement(self);
    }
}

impl Traverseable for ForStatement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.setup();
        if let Some(e) = &mut self.init { e.accept(v); }
        if let Some(e) = &mut self.each { e.accept(v); }
        if let Some(e) = &mut self.end  { e.accept(v); }
        self.stmt.accept(v);
        v.handle_for_statement(self);
    }
}

impl Traverseable for WhileStatement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.setup();
        self.condition.accept(v);
        self.stmt.accept(v);
        v.handle_while_statement(self);
    }
}

impl Traverseable for CompoundStatement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.setup();
        v.begin_compound_statement(self);
        if let Some(vec) = &mut self.stmts {
            for s in vec { s.accept(v); }
        }
        v.handle_compound_statement(self);
    }
}

impl Traverseable for JumpStatement {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.setup();
        if let Some(e) = &mut self.expr { e.accept(v) }
        v.handle_jump_statement(self);
    }
}

impl Traverseable for Expr {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        match self.etype {
            ExprType::Function(mut f)    => f.accept(v),
            ExprType::Access(mut a)      => a.accept(v),
            ExprType::Unary(mut u)       => u.accept(v),
            ExprType::Binary(mut b)      => b.accept(v),
            ExprType::Identifier(mut i)  => i.accept(v),
            ExprType::Integer(_)         => (),
            ExprType::Float(_)           => (),
        }
        v.handle_expr(self);
    }
}

impl Traverseable for FunctionCall {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.setup();
        if let Some(vec) = &mut self.args {
            for e in vec { e.accept(v); }
        }
        v.handle_function_call(self);        
    }
}

impl Traverseable for AccessExpr {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.setup();
        self.offset.accept(v);
        v.handle_access(self);
    }
}

impl Traverseable for UnaryExpr {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.setup();
        self.expr.accept(v);
        v.handle_unary(self);
    }
}

impl Traverseable for BinaryExpr {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.setup();
        self.left.accept(v);
        self.right.accept(v);
        v.handle_binary(self);
    }
}

impl Traverseable for Identifier {
    fn accept<T: Visitor>(&mut self, v: &mut T) {
        v.setup();
        v.handle_identifier(self);
    }
}