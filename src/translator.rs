use crate::ast;
use crate::ast::FunctionDeclaration;
use crate::ir;
use crate::ast::ExprType::*;
use crate::visitor::Visitor;
use crate::traverse::Traverseable;

struct Translator {
    nlabels: u32
}

impl Translator {
    pub fn new() -> Self { Self { nlabels: 0 } }
    pub fn translate(&mut self, m: &mut ast::Module) -> Vec::<ir::Statement> {
        let mut res = Vec::<ir::Statement>::new();
        for f in &m.functions {
            res.push(self.function_definition(f));
        }
        return res;
    }
    fn function_definition(&mut self, f: &FunctionDeclaration) -> ir::Statement {
        let mut stmts = self.compound_statement(&f.statement);
        stmts.insert(0,
            ir::Statement::Label(
                self.create_label()
            )
        );
        return ir::Statement::Seq(stmts);
    }
    fn statement(&mut self, s: &ast::Statement) -> Option<ir::Statement> {
        use ast::Statement::*;
        return match s {
            Declare  (ref d) => self.declare_statement(d),
            Expr     (ref e) => self.expr_statement(e),
            If       (ref i) => Some(self.if_statement(i)),
            For      (ref f) => Some(self.for_statement(f)),
            While    (ref w) => Some(self.while_statement(w)),
            Compound (ref c) => Some(self.compound_statement(c)),
            Jump     (ref j) => Some(self.jump_statment(j))
        }
    }
    fn declare_statement(&mut self, d: &ast::DeclareStatement) -> Option<ir::Statement> {
        match d.expr {
            None    => return None,
            Some(e) => return Some(ir::Statement::Move(
                Box::new(ir::Expr::Temp(d.id, d.kind)),
                Box::new(self.expr(e))
            ))
        }
    }
    fn compound_statement(&mut self, c: &ast::CompoundStatement) -> Vec<ir::Statement> {
        let mut stmts = Vec::<ir::Statement>::new();
        for stmt in &c.stmts {
            stmts.push(self.statement(stmt));
        }
        return stmts;
    }
    fn expr_statement(&mut self, e: &ast::ExprStatement) -> Option<ir::Statement> {
        return match e.expr {
            None     => None,
            Some(ex) => Some(ir::Statement::Expr(
                self.expression(ex)
            ))
        }
    }
    fn if_statement(&mut self, i: &ast::IfStatement) -> ir::Statement {
        let lt = self.create_label();
        let lf = self.create_label();
        let mut ret = vec![
            self.control(&i.condition, lt, lf),
            ir::Statement::Label(lt),
        ];
        match self.statement(&i.true_stmt) {
            None    => (),
            Some(s) => ret.push(s)
        }
        ret.push(ir::Statement::Label(lf));
        if let Some(s) = i.false_stmt {
            match self.statement(&s) {
                None    => (),
                Some(s) => ret.push(s)
            }
        }
        return ir::Statement::Seq(ret);
    }
    fn for_statement(&mut self, f: &ast::ForStatement) -> ir::Statement {
        let mut ret = Vec::<ir::Statement>::new();
        match f.init {
            None    => (),
            Some(s) => ret.push(self.expression(s))
        }
        let lt = self.create_label();
        let lb = self.create_label();
        let le = self.create_label();
        ret.push(ir::Statement::Label(lt));
        match f.end {
            None    => (),
            Some(s) => ret.push(self.control(
                self.expression(s), lb, le
            ))
        }
        ret.push(ir::Statement::Label(lb));
        match self.statement(&f.stmt) {
            None => (),
            Some(s) => ret.push(s)
        }
        match f.each {
            None    => (),
            Some(s) => ret.push(self.expression(s))
        }
        ret.push(ir::Statement::Jump(
            ir::Expr::Name(lt)
        ));
        ret.push(ir::Statement::Label(le));
        return ir::Statement::Seq(ret);
    }
    fn while_statement(&mut self, w: &ast::WhileStatement) -> ir::Statement {
        let lt = self.create_label();
        let lb = self.create_label();
        let le = self.create_label();
        let mut ret = vec![
            ir::Statement::Label(lt),
            self.control(&w.condition, lb, le),
            ir::Statement::Label(lb)
        ];
        match self.statement(&w.stmt) {
            None => (),
            Some(s) => ret.push(s)
        }
        ret.push(ir::Statement::Jump(
            ir::Expr::Name(lt)
        ));
        ret.push(ir::Statement::Label(le));
        return ir::Statement::Seq(ret);
    }
    fn jump_statement(&mut self, j: &ast::JumpStatement) -> ir::Statement {
        use ast::JumpOp::*;
        match j.jump_type {
            Continue => todo!(),
            Return => {
                match j.expr {
                    None    => ir::Statement::Return(None),
                    Some(s) => ir::Statement::Return(self.expression(s))
                }
            },
            Break => todo!(),
        }
    }
    fn control(&mut self, expr: &ast::Expr, t: ir::Label, f: ir::Label) -> ir::Statement {
        match expr.etype {
            Unary(u) => {
                use ast::UnaryOp::*;
                match u.unary_op {
                    Not => return self.control(
                        &u.expr, t, f
                    ),
                    _   => return ir::Statement::CJump(
                        Box::new(self.expression(expr)),
                        t, f
                    )
                }
            },
            Binary(b) => {
                use ast::BinaryOp::*;
                match b.binary_op {
                    And => {
                        let l1 = self.create_label();
                        return ir::Statement::Seq(vec![
                            self.control(&b.left, l1, f),
                            ir::Statement::Label(l1),
                            self.control(&b.right, t, f),
                        ]);
                    },
                    Or => {
                        let l1 = self.create_label();
                        return ir::Statement::Seq(vec![
                            self.control(&b.left, t, l1),
                            ir::Statement::Label(l1),
                            self.control(&b.right, t, f),
                        ]);
                    },
                    _   => ()
                }
            },
            Integer(i) => return ir::Statement::Jump(ir::Expr::Name(
                if i != 0 { t } else { f }
            )),
            Identifier(i) => {
                assert!(i.kind.unwrap() == ast::Kind::int(),
                    "Identifier is not an integer!"
                );
                return ir::Statement::CJump(
                    Box::new(ir::Expr::Temp(i.id)),
                    t, f
                )
            },
            Float(_) => panic!("Floats found in conditional!"),
            tp => assert!(tp.kind.unwrap() == ast::Kind::int(),
                "Conditional is not an integer!"
            )
        }
        return ir::Statement::CJump(
            Box::new(self.expression(expr)),
            t, f
        )
    }
    fn expression(&mut self, e: &ast::Expr) -> ir::Expr {
        use ast::ExprType::*;
        match e.etype {
            Function(_) => todo!(),
            Access(a) => todo!(),
            Unary(u) => return ir::Expr::UnOp(
                todo!(), /* some annoying op -> op code */
                Box::new(self.expression(&u.expr)),
            ),
            Binary(b) => return ir::Expr::BinOp(
                use ast::BinaryOp::*;
                match b.binary_op {

                }
                Box::new(self.expression(&b.left))
                todo!(), /* some annoying op -> op code */
                Box::new(self.expression(&b.right)),
            ),
            Integer(i) => return ir::Expr::Const(
                ir::Primitive::Int(i)
            ),
            Float(f) => return ir::Expr::Const(
                ir::Primitive::Float(f)
            ),
            Identifier(i) => return ir::Expr::Temp(i.id),
        }
    }
    fn map_bin(b: ast::BinaryOp) -> ir::Operator {
        use ast::BinaryOp::*;
        return match b {
            Mul => ir::Operator::Mul,
            Div => ir::Operator::Div,
            Add => ir::Operator::Add,
            Sub => ir::Operator::Sub,
            Leq => ir::Operator::Leq,
            Geq => ir::Operator::Geq,
            Lt => ir::Operator::Lt,
            Gt => ir::Operator::Gt,
            Eq => ir::Operator::Eq,
            Peq => ir::Operator::Peq,
            Teq => ir::Operator::Teq,
            Deq => ir::Operator::Deq,
            Seq => ir::Operator::Seq,
            Neq => ir::Operator::Neq,
            Or => ir::Operator::Or,
            And => ir::Operator::And,
            Assign => ir::Operator::Assign,
        }
    }
    fn create_label(&mut self) -> ir::Label {
        self.nlabels += 1;
        return ir::Label { id: self.nlabels - 1 }
    }
}