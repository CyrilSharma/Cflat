use crate::ast;
use crate::allocator::ir_arena;
use crate::ast::FunctionDeclaration;
use crate::ir::{self, Operator};

use bumpalo::{Bump, boxed::Box};

pub struct Translator<'l> {
    nlabels:     u32,
    loop_starts: Vec<ir::Label>,
    loop_ends:   Vec<ir::Label>,
    arena:       &'l Bump
}
impl<'l> Translator<'l> {
    pub fn new() -> Self { 
        Self {
            nlabels:     0,
            loop_starts: Vec::new(),
            loop_ends:   Vec::new(),
        } 
    }
    pub fn translate(&mut self, m: &mut ast::Module) -> Vec::<&mut ir::Statement> {
        self.nlabels = m.functions.len() as u32; // function ids are their labels.
        let mut res = Vec::<&mut ir::Statement>::new();
        for f in &m.functions {
            match self.function_declaration(f) {
                None => (),
                Some(s) => res.push(Box::new_in(s))
            }
        }
        return res;
    }
    fn function_declaration(&mut self, f: &FunctionDeclaration) -> Option<ir::Statement> {
        let mut ret = vec![Box::new_in(ir::Statement::Label(f.id))];
        match self.statement(&f.stmt) {
            None => return None,
            Some(s) => ret.push(Box::new_in(s))
        }
        return Some(ir::Statement::Seq(ret));
    }
    /*----------------STATEMENTS--------------------*/
    fn statement(&mut self, s: &ast::Statement) -> Option<ir::Statement> {
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
    fn declare_statement(&mut self, d: &ast::DeclareStatement) -> Option<ir::Statement> {
        match &d.val {
            None    => return None,
            Some(e) => return Some(ir::Statement::Move(
                Box::new_in(ir::Expr::Temp(d.id)),
                Box::new_in(self.expression(e))
            ))
        }
    }
    fn compound_statement(&mut self, c: &ast::CompoundStatement) -> ir::Statement {
        let mut stmts = Vec::<&mut ir::Statement>::new();
        for stmt in &c.stmts {
            match self.statement(stmt) {
                None    => (),
                Some(s) => stmts.push(Box::new_in(s))
            }
        }
        return ir::Statement::Seq(stmts);
    }
    fn expr_statement(&mut self, e: &ast::ExprStatement) -> Option<ir::Statement> {
        return match &e.expr {
            None     => None,
            Some(ex) => Some(ir::Statement::Expr(
                Box::new_in(self.expression(ex))
            ))
        }
    }
    fn if_statement(&mut self, i: &ast::IfStatement) -> ir::Statement {
        let lt = self.create_label();
        let lf = self.create_label();
        let mut ret = vec![
            Box::new_in(self.control(&i.condition, lt, lf)),
            Box::new_in(ir::Statement::Label(lt)),
        ];
        match self.statement(&i.true_stmt) {
            None    => (),
            Some(s) => ret.push(Box::new_in(s))
        }
        ret.push(Box::new_in(ir::Statement::Label(lf)));
        if let Some(s) = &i.false_stmt {
            match self.statement(&s) {
                None    => (),
                Some(s) => ret.push(Box::new_in(s))
            }
        }
        return ir::Statement::Seq(ret);
    }
    fn for_statement(&mut self, f: &ast::ForStatement) -> ir::Statement {
        let mut ret = Vec::<&mut ir::Statement>::new();
        match self.statement(&f.init) {
            None    => (),
            Some(s) => ret.push(Box::new_in(s))
        }
        let lt = self.create_label();
        let lb = self.create_label();
        let le = self.create_label();
        self.loop_starts.push(lt);
        self.loop_ends.push(lb);

        ret.push(Box::new_in(ir::Statement::Label(lt)));
        match &f.cond {
            None => (),
            Some(e) => ret.push(Box::new_in(
                self.control(e, lb, le)
            ))
        }
        ret.push(Box::new_in(ir::Statement::Label(lb)));
        match self.statement(&f.stmt) {
            None => (),
            Some(s) => ret.push(Box::new_in(s))
        }
        match &f.each {
            None    => (),
            Some(e) => ret.push(Box::new_in(
                ir::Statement::Expr(Box::new_in(
                    self.expression(e)
                ))
            ))
        }
        ret.push(Box::new_in(ir::Statement::Jump(lt)));
        ret.push(Box::new_in(ir::Statement::Label(le)));
        self.loop_starts.pop();
        self.loop_ends.pop();
        return ir::Statement::Seq(ret);
    }
    fn while_statement(&mut self, w: &ast::WhileStatement) -> ir::Statement {
        let lt = self.create_label();
        let lb = self.create_label();
        let le = self.create_label();
        self.loop_starts.push(lt);
        self.loop_ends.push(lb);

        let mut ret = vec![
            Box::new_in(ir::Statement::Label(lt)),
            Box::new_in(self.control(&w.condition, lb, le)),
            Box::new_in(ir::Statement::Label(lb))
        ];
        match self.statement(&w.stmt) {
            None => (),
            Some(s) => ret.push(Box::new_in(s))
        }
        ret.push(Box::new_in(ir::Statement::Jump(lt)));
        ret.push(Box::new_in(ir::Statement::Label(le)));

        self.loop_starts.pop();
        self.loop_ends.pop();
        return ir::Statement::Seq(ret);
    }
    fn jump_statement(&mut self, j: &ast::JumpStatement) -> ir::Statement {
        use ast::JumpOp::*;
        match j.jump_type {
            Continue => self._continue(),
            Return => self._return(&j.expr),
            Break => self._break()
        }
    }
    fn _continue(&mut self) -> ir::Statement {
        match self.loop_starts.iter().last() {
            None    => panic!("You done goof"),
            Some(l) => return ir::Statement::Jump(*l)
        }
    }
    fn _return(&mut self, e: &Option<Box<ast::Expr>>) -> ir::Statement {
        match e {
            None    => ir::Statement::Return(None),
            Some(s) => ir::Statement::Return(Some(
                Box::new_in(self.expression(&s))
            ))
        }
    }
    fn _break(&mut self) -> ir::Statement {
        match self.loop_ends.iter().last() {
            None    => panic!("You done goof"),
            Some(l) => return ir::Statement::Jump(*l)
        }
    }
    /*----------------CONTROL--------------------*/
    fn control(&mut self, expr: &ast::Expr, t: ir::Label, f: ir::Label) -> ir::Statement {
        use ast::Expr::*;
        let res = match expr {
            Unary(u) => self.control_unary(&u, t, f),
            Binary(b) => self.control_binary(&b, t, f),
            Integer(i) => Some(ir::Statement::Jump(
                if *i != 0 { t } else { f }
            )),
            Ident(i) => Some(ir::Statement::CJump(
                Box::new_in(ir::Expr::Temp(i.id)),
                t, f
            )),
            _ => None
        };
        return match res {
            None    => ir::Statement::CJump(
                Box::new_in(self.expression(expr)),
                t, f
            ),
            Some(s) => s
        }
    }
    fn control_unary(&mut self, u: &ast::UnaryExpr, t: ir::Label, f: ir::Label)
        -> Option<ir::Statement> {
        use ast::UnaryOp::*;
        return match u.unary_op {
            Not => Some(self.control(
                &u.expr, f, t
            )),
            _ => None
        }
    }
    fn control_binary(&mut self, b: &ast::BinaryExpr, t: ir::Label, f: ir::Label)
        -> Option<ir::Statement> {
        use ast::BinaryOp::*;
        return match b.binary_op {
            And => {
                let l1 = self.create_label();
                Some(ir::Statement::Seq(vec![
                    Box::new_in(self.control(&b.left, l1, f)),
                    Box::new_in(ir::Statement::Label(l1)),
                    Box::new_in(self.control(&b.right, t, f)),
                ]))
            },
            Or => {
                let l1 = self.create_label();
                Some(ir::Statement::Seq(vec![
                    Box::new_in(self.control(&b.left, t, l1)),
                    Box::new_in(ir::Statement::Label(l1)),
                    Box::new_in(self.control(&b.right, t, f)),
                ]))
            },
            _   => None
        }
    }
    /*----------------EXPRESSIONS--------------------*/
    fn expression(&mut self, e: &ast::Expr) -> ir::Expr {
        use ast::Expr::*;
        match e {
            Function(f) => self.function(&f),
            Access(a) => self.access(&a),
            Unary(u) => self.unary(&u),
            Binary(b) => self.binary(&b),
            Integer(i) => return ir::Expr::Const(
                ir::Primitive::Int(*i)
            ),
            Float(f) => return ir::Expr::Const(
                ir::Primitive::Float(*f)
            ),
            Ident(i) => return ir::Expr::Temp(i.id),
        }
    }
    fn function(&mut self, f: &ast::FunctionCall) -> ir::Expr {
        let mut v = Vec::<&mut ir::Expr>::new();
        for exp in &f.args {
            v.push(Box::new_in(
                self.expression(exp)
            ));
        }
        return ir::Expr::Call(f.id, v);
    }
    fn access(&mut self, a: &ast::AccessExpr) -> Box<'l, ir::Expr> {
        let mut prod: u32 = 1;
        let mut root: Option<ir::Expr> = None;
        for i in (0..a.offsets.len()).rev() {
            let mul = ir::Expr::BinOp(
                Box::new_in(ir::Expr::Const(
                    ir::Primitive::Int(
                        8 * prod as i32
                    )
                )),
                Operator::Mul,
                Box::new_in(self.expression(&a.offsets[i]))
            );
            root = match root {
                None    => Some(mul),
                Some(e) => Some(ir::Expr::BinOp(
                    Box::new_in(e),
                    Operator::Add,
                    Box::new_in(mul)
                ))
            };
            prod *= a.sizes[i];
        }
        let t = Box::new_in(ir::Expr::Temp(a.id), self.arena);
        let inc = Box::new_in(root.unwrap(), self.arena);
        let exp = Box::new_in(ir::Expr::BinOp(t, Operator::Add, inc), self.arena);
        return Box::new_in(ir::Expr::Mem(exp), self.arena);
    }
    fn unary(&mut self, u: &ast::UnaryExpr) -> Box<'l, ir::Expr> {
        use ast::UnaryOp::*;
        let op = match u.unary_op {
            Not  => ir::Operator::Not,
            Neg  => ir::Operator::Neg,
            Star => {
                let e = Box::new_in(self.expression(&u.expr), self.arena);
                return Box::new_in(ir::Expr::Mem(e), self.arena);
            }
            Address => {
                let e = Box::new_in(self.expression(&u.expr), self.arena);
                return Box::new_in(ir::Expr::Address(e), self.arena);
            }
        };
        let e = Box::new_in(self.expression(&u.expr), self.arena);
        return Box::new_in(ir::Expr::UnOp(op, e), self.arena);
    }
    fn binary(&mut self, b: &ast::BinaryExpr) -> Box<'l, ir::Expr> {
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
        let l = Box::new_in(self.expression(&b.left), self.arena);
        let r = Box::new_in(self.expression(&b.right), self.arena);
        return Box::new_in(ir::Expr::BinOp(l, op, r), self.arena);
    }
    fn assign(&mut self, b: &ast::BinaryExpr) -> Box<'l, ir::Expr> {
        use ast::BinaryOp::*;
        let stmt = match b.binary_op {
            Peq | Teq | Seq | Deq => {
                let op = match b.binary_op {
                    Peq => ir::Operator::Add,
                    Teq => ir::Operator::Mul,
                    Seq => ir::Operator::Sub,
                    Deq => ir::Operator::Div,
                    _   => unreachable!()
                };
                let e = Box::new_in(self.expression(&b.left), self.arena);
                let l = Box::new_in(self.expression(&b.left), self.arena);
                let r = Box::new_in(self.expression(&b.right), self.arena);
                let o = Box::new_in(ir::Expr::BinOp(l, op, r), self.arena);
                Box::new_in(ir::Statement::Move(e, o), self.arena)
            },
            Assign => {
                let l = Box::new_in(self.expression(&b.left), self.arena);
                let r = Box::new_in(self.expression(&b.right), self.arena);
                Box::new_in(ir::Statement::Move(l, r), self.arena)
            },
            _ => unreachable!()
        };
        let e = Box::new_in(self.expression(&b.left), self.arena);
        return Box::new_in(ir::Expr::ESeq(stmt, e), self.arena);
    }
    fn create_label(&mut self) -> ir::Label {
        self.nlabels += 1;
        return self.nlabels - 1;
    }
}