use crate::ast::*;
pub struct Printer { count: u32 }
impl Printer {
    pub fn new() -> Self { Self{count: 0} }
    pub fn print_module(&mut self, m: &Module) {
        println!("digraph AST {{");
        let idx = self.count;
        self.add_label("Module");
        for f in &m.functions {
            self.add_edge(idx, self.count);
            self.print_function_declaration(f);
        }
        println!("}}");
    }
    fn print_function_declaration(&mut self, f: &FunctionDeclaration) {
        let idx = self.count;
        let mut kind_str = format!("{:?}", f.ret.prim);
        for _ in 0..f.ret.indirection { kind_str.push('*'); }
        self.add_label(&format!("Declare {kind_str} {}()", f.name));
        self.add_edge(idx, self.count);
        self.print_statement(&f.statement);
    }
    fn print_statement(&mut self, s: &Statement) {
        match s {
            Statement::Declare(d) => self.print_declare_statement(d),
            Statement::Expr(e) => self.print_expr_statement(e),
            Statement::If(i) => self.print_if_statement(i),
            Statement::For(f) => self.print_for_statement(f),
            Statement::While(w) => self.print_while_statement(w),
            Statement::Compound(c) => self.print_compound_statement(c),
            Statement::Jump(j) => self.print_jump_statement(j)
        }
    }
    fn print_declare_statement(&mut self, d: &DeclareStatement) {
        let idx = self.count;
        let mut kind_str = format!("{:?}", d.kind.prim);
        for _ in 0..d.kind.indirection { kind_str.push('*'); }
        self.add_label(&format!("Declare: {kind_str} {}", d.name));
        if let Some(e) = &d.val {
            self.add_edge(idx, self.count);
            self.print_expr(e);
        }
    }
    fn print_expr_statement(&mut self, e: &ExprStatement) {
        let idx = self.count;
        self.add_label("Expression Statement");
        if let Some(e) = &e.expr {
            self.add_edge(idx, self.count);
            self.print_expr(e);
        }
    }
    fn print_if_statement(&mut self, i: &IfStatement) {
        let idx = self.count;
        self.add_label("If");

        self.add_edge(idx, self.count);
        self.print_expr(&i.condition);

        self.add_edge(idx, self.count);
        self.print_statement(&i.true_stmt);

        if let Some(e) = &i.false_stmt {
            self.add_edge(idx, self.count);
            self.print_statement(e);
        }
    }
    fn print_for_statement(&mut self, f: &ForStatement) {
        let idx = self.count;
        self.add_label("For");
        if let Some(e) = &f.init {
            self.add_edge(idx, self.count);
            self.print_expr(e);
        }
        if let Some(e) = &f.each {
            self.add_edge(idx, self.count);
            self.print_expr(e);
        }
        if let Some(e) = &f.end {
            self.add_edge(idx, self.count);
            self.print_expr(e);
        }
        self.add_edge(idx, self.count);
        self.print_statement(&f.stmt);
    }
    fn print_while_statement(&mut self, w: &WhileStatement) {
        let idx = self.count;
        self.add_label("While");
        
        self.add_edge(idx, self.count);
        self.print_expr(&w.condition);

        self.add_edge(idx, self.count);
        self.print_statement(&w.stmt);
    }
    fn print_compound_statement(&mut self, c: &CompoundStatement) {
        let idx = self.count;
        self.add_label("Compound Statement");
        if let Some(v) = &c.stmts {
            for s in v {
                self.add_edge(idx, self.count);
                self.print_statement(s);
            }
        }
    }
    fn print_jump_statement(&mut self, j: &JumpStatement) {
        let idx = self.count;
        self.add_label(&format!("{:?}", j.jump_type));
        if let Some(e) = &j.expr {
            self.add_edge(idx, self.count);
            self.print_expr(e);
        }
    }
    fn print_expr(&mut self, e: &Expr) {
        match e {
            Expr::Function(f) => self.print_function(f),
            Expr::Access(a) => self.print_access(a),
            Expr::Unary(u) => self.print_unary(u),
            Expr::Binary(b) => self.print_binary(b),
            Expr::Integer(i) => self.print_integer(*i),
            Expr::Float(f) => self.print_float(*f),
            Expr::Identifier(i) => self.print_identifier(i)
        }
    }
    fn print_function(&mut self, f: &FunctionCall) {
        let idx = self.count;
        self.add_label(&format!("Call Function: {}", f.name));
        if let Some(v) = &f.args {
            for e in v {
                self.add_edge(idx, self.count);
                self.print_expr(e); 
            }
        }
    }
    fn print_access(&mut self, a: &AccessExpr) {
        let idx = self.count;
        self.add_label(&format!("Access: {}", a.name));
        self.add_edge(idx, self.count);
        self.print_expr(&a.offset);
    }
    fn print_unary(&mut self, u: &UnaryExpr) {
        let idx = self.count;
        self.add_label(&format!("Unary: {:?}", u.unary_op));
        self.add_edge(idx, self.count);
        self.print_expr(&u.expr);
    }
    fn print_binary(&mut self, b: &BinaryExpr) {
        let idx = self.count;
        self.add_label(&format!("Binary: {:?}", b.binary_op));
        self.add_edge(idx, self.count);
        self.print_expr(&b.left);
        self.add_edge(idx, self.count);
        self.print_expr(&b.right);
    }
    fn print_integer(&mut self, i: i32) {
        self.add_label(&format!("Integer: {}", i));
    }
    fn print_float(&mut self, f: f32) {
        self.add_label(&format!("Float: {}", f));
    }
    fn print_identifier(&mut self, s: &str) {
        self.add_label(&format!("Identifier: {}", s));
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