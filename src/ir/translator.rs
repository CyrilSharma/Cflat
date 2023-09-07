use crate::ast::ast::{self, FunctionDeclaration};
use super::ir::{self, Operator};
use crate::registry::Registry;

pub struct Translator<'l> {
    loop_starts: Vec<ir::Label>,
    loop_ends:   Vec<ir::Label>,
    reg:         &'l mut Registry
}

impl<'l> Translator<'l> {
    pub fn new(registry: &'l mut Registry) -> Self { 
        Self {
            loop_starts: Vec::new(),
            loop_ends:   Vec::new(),
            reg:         registry
        } 
    }
    pub fn translate(&mut self, m: &mut ast::Module) -> Vec::<Box<ir::Statement>> {
        self.reg.nfuncs  = m.functions.len() as u32;
        self.reg.nlabels = m.functions.len() as u32; // function ids are their labels.
        let mut res = Vec::<Box<ir::Statement>>::new();
        for f in &m.functions {
            match self.function_declaration(f) {
                None => (),
                Some(s) => res.push(s)
            }
        }
        return res;
    }
    fn function_declaration(&mut self, f: &FunctionDeclaration) -> Option<Box<ir::Statement>> {
        let ids: Vec<ir::ID> = f.params.iter().map(|p| p.id).collect();
        let mut ret = vec![Box::new(
            ir::Statement::Function(f.id, ids)
        )];
        match self.statement(&f.stmt) {
            None => return None,
            Some(s) => ret.push(s)
        }
        return Some(Box::new(ir::Statement::Seq(ret)));
    }
    /*----------------STATEMENTS--------------------*/
    fn statement(&mut self, s: &ast::Statement) -> Option<Box<ir::Statement>> {
        use ast::Statement::*;
        return match s {
            Declare  (ref d) => self.declare_statement(d),
            Expr     (ref e) => self.expr_statement(e),
            If       (ref i) => Some(self.if_statement(i)),
            For      (ref f) => Some(self.for_statement(f)),
            While    (ref w) => Some(self.while_statement(w)),
            Compound (ref c) => Some(self.compound_statement(c)),
            Jump     (ref j) => Some(self.jump_statement(j))
        }
    }
    fn declare_statement(&mut self, d: &ast::DeclareStatement) -> Option<Box<ir::Statement>> {
        match &d.val {
            None    => return None,
            Some(e) => return Some(Box::new(ir::Statement::Move(
                Box::new(ir::Expr::Temp(d.id)),
                self.expression(e)
            )))
        }
    }
    fn compound_statement(&mut self, c: &ast::CompoundStatement) -> Box<ir::Statement> {
        let mut stmts = Vec::<Box<ir::Statement>>::new();
        for stmt in &c.stmts {
            match self.statement(stmt) {
                None    => (),
                Some(s) => stmts.push(s)
            }
        }
        return Box::new(ir::Statement::Seq(stmts));
    }
    fn expr_statement(&mut self, e: &ast::ExprStatement) -> Option<Box<ir::Statement>> {
        return match &e.expr {
            None     => None,
            Some(ex) => Some(Box::new(ir::Statement::Expr(
                self.expression(ex)
            )))
        }
    }
    fn if_statement(&mut self, i: &ast::IfStatement) -> Box<ir::Statement> {
        let lt = self.create_label();
        let lf = self.create_label();
        let mut ret = vec![
            self.control(&i.condition, lt, lf),
            Box::new(ir::Statement::Label(lt)),
        ];
        match self.statement(&i.true_stmt) {
            None    => (),
            Some(s) => ret.push(s)
        }
        ret.push(Box::new(ir::Statement::Label(lf)));
        if let Some(s) = &i.false_stmt {
            match self.statement(&s) {
                None    => (),
                Some(s) => ret.push(s)
            }
        }
        return Box::new(ir::Statement::Seq(ret));
    }
    fn for_statement(&mut self, f: &ast::ForStatement) -> Box<ir::Statement> {
        let mut ret = Vec::<Box<ir::Statement>>::new();
        match self.statement(&f.init) {
            None    => (),
            Some(s) => ret.push(s)
        }
        let lt = self.create_label();
        let lb = self.create_label();
        let le = self.create_label();
        self.loop_starts.push(lt);
        self.loop_ends.push(lb);

        ret.push(Box::new(ir::Statement::Label(lt)));
        match &f.cond {
            None => (),
            Some(e) => ret.push(self.control(e, lb, le))
        }
        ret.push(Box::new(ir::Statement::Label(lb)));
        match self.statement(&f.stmt) {
            None => (),
            Some(s) => ret.push(s)
        }
        match &f.each {
            None    => (),
            Some(e) => ret.push(Box::new(ir::Statement::Expr(
                self.expression(e)
            )))
        }
        ret.push(Box::new(ir::Statement::Jump(lt)));
        ret.push(Box::new(ir::Statement::Label(le)));

        self.loop_starts.pop();
        self.loop_ends.pop();
        return Box::new(ir::Statement::Seq(ret));
    }
    fn while_statement(&mut self, w: &ast::WhileStatement) -> Box<ir::Statement> {
        let lt = self.create_label();
        let lb = self.create_label();
        let le = self.create_label();
        self.loop_starts.push(lt);
        self.loop_ends.push(lb);

        let mut ret = vec![
            Box::new(ir::Statement::Label(lt)),
            self.control(&w.condition, lb, le),
            Box::new(ir::Statement::Label(lb))
        ];
        match self.statement(&w.stmt) {
            None => (),
            Some(s) => ret.push(s)
        }
        ret.push(Box::new(ir::Statement::Jump(lt)));
        ret.push(Box::new(ir::Statement::Label(le)));

        self.loop_starts.pop();
        self.loop_ends.pop();
        return Box::new(ir::Statement::Seq(ret));
    }
    fn jump_statement(&mut self, j: &ast::JumpStatement) -> Box<ir::Statement> {
        use ast::JumpOp::*;
        match j.jump_type {
            Continue => self._continue(),
            Return => self._return(&j.expr),
            Break => self._break()
        }
    }
    fn _continue(&mut self) -> Box<ir::Statement> {
        match self.loop_starts.iter().last() {
            None    => panic!("You done goof"),
            Some(l) => Box::new(ir::Statement::Jump(*l))
        }
    }
    fn _return(&mut self, e: &Option<Box<ast::Expr>>) -> Box<ir::Statement> {
        return Box::new(match e {
            None    => ir::Statement::Return(None),
            Some(s) => ir::Statement::Return(Some(
                self.expression(&s)
            ))
        })
    }
    fn _break(&mut self) -> Box<ir::Statement> {
        match self.loop_ends.iter().last() {
            None    => panic!("You done goof"),
            Some(l) => return Box::new(ir::Statement::Jump(*l))
        }
    }
    /*----------------CONTROL--------------------*/
    fn control(&mut self, expr: &ast::Expr, t: ir::Label, f: ir::Label)
        -> Box<ir::Statement> {
        use ast::Expr::*;
        let res = match expr {
            Unary(u) => self.control_unary(&u, t, f),
            Binary(b) => self.control_binary(&b, t, f),
            Integer(i) => Some(Box::new(ir::Statement::Jump(
                if *i != 0 { t } else { f }
            ))),
            Ident(i) => Some(Box::new(ir::Statement::CJump(
                Box::new(ir::Expr::Temp(i.id)),
                t, f
            ))),
            _ => None
        };
        return match res {
            None    => Box::new(ir::Statement::CJump(
                self.expression(expr),
                t, f
            )),
            Some(s) => s
        }
    }
    fn control_unary(&mut self, u: &ast::UnaryExpr, t: ir::Label, f: ir::Label)
        -> Option<Box<ir::Statement>> {
        use ast::UnaryOp::*;
        return match u.unary_op {
            Not => Some(self.control(
                &u.expr, f, t
            )),
            _ => None
        }
    }
    fn control_binary(&mut self, b: &ast::BinaryExpr, t: ir::Label, f: ir::Label)
        -> Option<Box<ir::Statement>> {
        use ast::BinaryOp::*;
        return match b.binary_op {
            And => {
                let l1 = self.create_label();
                Some(Box::new(ir::Statement::Seq(vec![
                    self.control(&b.left, l1, f),
                    Box::new(ir::Statement::Label(l1)),
                    self.control(&b.right, t, f),
                ])))
            },
            Or => {
                let l1 = self.create_label();
                Some(Box::new(ir::Statement::Seq(vec![
                    self.control(&b.left, t, l1),
                    Box::new(ir::Statement::Label(l1)),
                    self.control(&b.right, t, f),
                ])))
            },
            _   => None
        }
    }
    /*----------------EXPRESSIONS--------------------*/
    fn expression(&mut self, e: &ast::Expr) -> Box<ir::Expr> {
        use ast::Expr::*;
        match e {
            Function(f) => self.function(&f),
            Access(a) => self.access(&a),
            Unary(u) => self.unary(&u),
            Binary(b) => self.binary(&b),
            Integer(i) => Box::new(ir::Expr::Const(
                ir::Primitive::Int(*i as i64)
            )),
            Float(f) => Box::new(ir::Expr::Const(
                ir::Primitive::Float(*f as f64)
            )),
            Ident(i) => Box::new(ir::Expr::Temp(i.id)),
        }
    }
    fn function(&mut self, f: &ast::FunctionCall) -> Box<ir::Expr> {
        let mut v = Vec::<Box<ir::Expr>>::new();
        for exp in &f.args {
            v.push(self.expression(exp));
        }
        return Box::new(ir::Expr::Call(f.id, v));
    }
    fn access(&mut self, a: &ast::AccessExpr) -> Box<ir::Expr> {
        let mut prod: u32 = 1;
        let t = ir::Expr::Temp(a.id);
        let mut root: Option<ir::Expr> = None;
        for i in (0..a.offsets.len()).rev() {
            let mul = ir::Expr::BinOp(
                Box::new(ir::Expr::Const(
                    ir::Primitive::Int(
                        8 * prod as i64
                    )
                )),
                Operator::Mul,
                self.expression(&a.offsets[i])
            );
            root = match root {
                None    => Some(mul),
                Some(e) => Some(ir::Expr::BinOp(
                    Box::new(e),
                    Operator::Add,
                    Box::new(mul)
                ))
            };
            prod *= a.sizes[i];
        }
        let exp = Box::new(ir::Expr::BinOp(
            Box::new(t),
            Operator::Add,
            Box::new(root.unwrap())
        ));
        return Box::new(ir::Expr::Mem(exp));
    }
    fn unary(&mut self, u: &ast::UnaryExpr) -> Box<ir::Expr> {
        use ast::UnaryOp::*;
        let op = match u.unary_op {
            Not  => ir::Operator::Not,
            Neg  => ir::Operator::Neg,
            Star => return Box::new(ir::Expr::Mem(
                self.expression(&u.expr)
            )),
            Address => return Box::new(ir::Expr::Address(
                self.expression(&u.expr)
            ))
        };
        return Box::new(ir::Expr::UnOp(op,
            self.expression(&u.expr)
        ));
    }
    fn binary(&mut self, b: &ast::BinaryExpr) -> Box<ir::Expr> {
        use ast::BinaryOp::*;
        let op = match b.binary_op {
            Mul    => ir::Operator::Mul,
            Div    => ir::Operator::Div,
            Add    => ir::Operator::Add,
            Sub    => ir::Operator::Sub,
            Leq    => ir::Operator::Leq,
            Geq    => ir::Operator::Geq,
            Lt     => ir::Operator::Lt,
            Gt     => ir::Operator::Gt,
            Eq     => ir::Operator::Eq,
            Or     => ir::Operator::Or,
            And    => ir::Operator::And,
            Peq | Teq | Deq |
            Seq | Neq | Assign  
                => return self.assign(b),
        };
        return Box::new(ir::Expr::BinOp(
            self.expression(&b.left),
            op,
            self.expression(&b.right),
        ));
    }
    fn assign(&mut self, b: &ast::BinaryExpr) -> Box<ir::Expr> {
        use ast::BinaryOp::*;
        // Ideally we'd implement Clone for Expr, but
        // b.left should only be an access or temp,
        // so it's not terribly expensive.
        let stmt = Box::new(match b.binary_op {
            Peq | Teq | Seq | Deq => {
                let op = match b.binary_op {
                    Peq => ir::Operator::Add,
                    Teq => ir::Operator::Mul,
                    Seq => ir::Operator::Sub,
                    Deq => ir::Operator::Div,
                    _   => unreachable!()
                };
                ir::Statement::Move(
                    self.expression(&b.left),
                    Box::new(ir::Expr::BinOp(
                        self.expression(&b.left),
                        op,
                        self.expression(&b.right)
                    ))
                )
            },
            Assign => ir::Statement::Move(
                self.expression(&b.left),
                self.expression(&b.right)
            ),
            _ => unreachable!()
        });
        return Box::new(ir::Expr::ESeq(
            stmt, self.expression(&b.left)
        ));
    }
    fn create_label(&mut self) -> ir::Label {
        self.reg.nlabels += 1;
        return self.reg.nlabels - 1;
    }
}