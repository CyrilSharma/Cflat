use super::ast::*;
pub struct Printer { count: u32 }
impl Printer {
    pub fn new() -> Self { Self{count: 0} }
    pub fn print(&mut self, m: &Module) {
        println!("digraph AST {{");
        let idx = self.count;
        self.add_label("Module");
        for f in &m.functions {
            self.add_edge(idx, self.count);
            self.function_declaration(f);
        }
        println!("}}");
    }
    fn function_declaration(&mut self, f: &FunctionDeclaration) {
        let idx = self.count;
        self.add_label(&format!("Declare {} {}({})", f.ret, f.name, f.id));
        self.add_edge(idx, self.count);
        self.statement(&f.stmt);
    }
    fn statement(&mut self, s: &Statement) {
        match s {
            Statement::Declare(d) => self.declare_statement(d),
            Statement::Expr(e) => self.expr_statement(e),
            Statement::If(i) => self.if_statement(i),
            Statement::For(f) => self.for_statement(f),
            Statement::While(w) => self.while_statement(w),
            Statement::Compound(c) => self.compound_statement(c),
            Statement::Jump(j) => self.jump_statement(j),
            Statement::Asm(a)  => self.asm_statement(a)
        }
    }
    fn asm_statement(&mut self, _a: &AsmStatement) {
        self.add_label(&format!("Asm"));
    }
    fn declare_statement(&mut self, d: &DeclareStatement) {
        let idx = self.count;
        self.add_label(&format!("Declare: {} {}({})", d.kind, d.name, d.id));
        if let Some(e) = &d.val {
            self.add_edge(idx, self.count);
            self.expr(e);
        }
    }
    fn expr_statement(&mut self, e: &ExprStatement) {
        let idx = self.count;
        self.add_label("Expression Statement");
        if let Some(e) = &e.expr {
            self.add_edge(idx, self.count);
            self.expr(e);
        }
    }
    fn if_statement(&mut self, i: &IfStatement) {
        let idx = self.count;
        self.add_label("If");

        self.add_edge(idx, self.count);
        self.expr(&i.condition);

        self.add_edge(idx, self.count);
        self.statement(&i.true_stmt);

        if let Some(e) = &i.false_stmt {
            self.add_edge(idx, self.count);
            self.statement(e);
        }
    }
    fn for_statement(&mut self, f: &ForStatement) {
        let idx = self.count;
        self.add_label("For");
        self.add_edge(idx, self.count);
        self.statement(&f.init);
        if let Some(e) = &f.cond {
            self.add_edge(idx, self.count);
            self.expr(e);
        }
        if let Some(e) = &f.each {
            self.add_edge(idx, self.count);
            self.expr(e);
        }
        self.add_edge(idx, self.count);
        self.statement(&f.stmt);
    }
    fn while_statement(&mut self, w: &WhileStatement) {
        let idx = self.count;
        self.add_label("While");
        
        self.add_edge(idx, self.count);
        self.expr(&w.condition);

        self.add_edge(idx, self.count);
        self.statement(&w.stmt);
    }
    fn compound_statement(&mut self, c: &CompoundStatement) {
        let idx = self.count;
        self.add_label("Compound Statement");
        for s in &c.stmts {
            self.add_edge(idx, self.count);
            self.statement(s);
        }
    }
    fn jump_statement(&mut self, j: &JumpStatement) {
        let idx = self.count;
        self.add_label(&format!("{:?}", j.jump_type));
        if let Some(e) = &j.expr {
            self.add_edge(idx, self.count);
            self.expr(e);
        }
    }
    fn expr(&mut self, e: &Expr) {
        match e {
            Expr::Function(f) => self.function(f),
            Expr::Access(a) => self.access(a),
            Expr::Unary(u) => self.unary(u),
            Expr::Binary(b) => self.binary(b),
            Expr::Integer(i) => self.integer(*i),
            Expr::Float(f) => self.float(*f),
            Expr::Ident(i) => self.identifier(i)
        }
    }
    fn function(&mut self, f: &FunctionCall) {
        let idx = self.count;
        self.add_label(&format!("Call Function: {}({})", f.name, f.id));
        for e in &f.args {
            self.add_edge(idx, self.count);
            self.expr(e); 
        }
    }
    fn access(&mut self, a: &AccessExpr) {
        let idx = self.count;
        self.add_label(&format!("Access: {}", a.name));
        for e in &a.offsets {
            self.add_edge(idx, self.count);
            self.expr(e);
        }
    }
    fn unary(&mut self, u: &UnaryExpr) {
        let idx = self.count;
        self.add_label(&format!("Unary: {:?}", u.unary_op));
        self.add_edge(idx, self.count);
        self.expr(&u.expr);
    }
    fn binary(&mut self, b: &BinaryExpr) {
        let idx = self.count;
        self.add_label(&format!("Binary: {:?}", b.binary_op));
        self.add_edge(idx, self.count);
        self.expr(&b.left);
        self.add_edge(idx, self.count);
        self.expr(&b.right);
    }
    fn integer(&mut self, i: i64) {
        self.add_label(&format!("Integer: {}", i));
    }
    fn float(&mut self, f: f64) {
        self.add_label(&format!("Float: {}", f));
    }
    fn identifier(&mut self, i: &Identifier) {
        self.add_label(&format!("Identifier: {} {}({})",
            match i.kind {
                None => "Unknown".to_string(),
                Some(k) => format!("{}", k)
            }, i.name, i.id
        ));
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