use crate::ast::{self, Statement, Identifier};
use crate::ast::FunctionDeclaration;
use crate::ir::{self, Operator};

struct Translator {
    ids:     u32,
    nlabels: u32,
    loop_starts: Vec<ir::Label>,
    loop_ends:   Vec<ir::Label>
}

impl Translator {
    pub fn new(count: u32) -> Self { 
        Self { 
            ids:         count,   
            nlabels:     0,
            loop_starts: Vec::new(),
            loop_ends:   Vec::new()
        } 
    }
    pub fn translate(&mut self, m: &mut ast::Module) -> Vec::<ir::Statement> {
        let mut res = Vec::<ir::Statement>::new();
        for f in &m.functions {
            res.push(self.function_definition(f));
        }
        return res;
    }
    fn function_definition(&mut self, f: &FunctionDeclaration) -> ir::Statement {
        return ir::Statement::Seq(vec![
            ir::Statement::Label(
                self.create_label()
            ),
            self.compound_statement(&f.stmt)
        ]);
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
            Jump     (ref j) => Some(self.jump_statement(j))
        }
    }
    fn declare_statement(&mut self, d: &ast::DeclareStatement) -> Option<ir::Statement> {
        match d.val {
            None    => return None,
            Some(e) => return Some(ir::Statement::Move(
                Box::new(ir::Expr::Temp(d.id)),
                Box::new(self.expression(&e))
            ))
        }
    }
    fn compound_statement(&mut self, c: &ast::CompoundStatement) -> ir::Statement {
        let mut stmts = Vec::<ir::Statement>::new();
        for stmt in &c.stmts {
            match self.statement(stmt) {
                None    => (),
                Some(s) => stmts.push(s)
            }
        }
        return ir::Statement::Seq(stmts);
    }
    fn expr_statement(&mut self, e: &ast::ExprStatement) -> Option<ir::Statement> {
        return match e.expr {
            None     => None,
            Some(ex) => Some(ir::Statement::Expr(
                Box::new(self.expression(&ex))
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
        match self.expr_statement(&f.init) {
            None    => (),
            Some(s) => ret.push(s)
        }
        let lt = self.create_label();
        let lb = self.create_label();
        let le = self.create_label();
        self.loop_starts.push(lt);
        self.loop_ends.push(lb);

        ret.push(ir::Statement::Label(lt));
        match &f.cond.expr {
            None    => (),
            Some(e) => ret.push(self.control(
                e, lb, le
            ))
        }
        ret.push(ir::Statement::Label(lb));
        match self.statement(&f.stmt) {
            None => (),
            Some(s) => ret.push(s)
        }
        match &f.each {
            None    => (),
            Some(e) => ret.push(ir::Statement::Expr(
                Box::new(self.expression(e))
            ))
        }
        ret.push(ir::Statement::Jump(
            ir::Expr::Name(lt)
        ));
        ret.push(ir::Statement::Label(le));

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

        self.loop_starts.pop();
        self.loop_ends.pop();
        return ir::Statement::Seq(ret);
    }
    fn jump_statement(&mut self, j: &ast::JumpStatement) -> ir::Statement {
        use ast::JumpOp::*;
        match j.jump_type {
            Continue => self._continue(),
            Return => self._return(&mut j.expr),
            Break => self._break()
        }
    }
    fn _continue(&mut self) -> ir::Statement {
        match self.loop_starts.iter().last() {
            None    => panic!("You done goof"),
            Some(l) => return ir::Statement::Jump(ir::Expr::Name(*l))
        }
    }
    fn _return(&mut self, e: &mut Option<Box<ast::Expr>>) -> ir::Statement {
        match e {
            None    => ir::Statement::Return(None),
            Some(s) => ir::Statement::Return(Some(
                Box::new(self.expression(&s))
            ))
        }
    }
    fn _break(&mut self) -> ir::Statement {
        match self.loop_ends.iter().last() {
            None    => panic!("You done goof"),
            Some(l) => return ir::Statement::Jump(ir::Expr::Name(*l))
        }
    }
    fn control(&mut self, expr: &ast::Expr, t: ir::Label, f: ir::Label) -> ir::Statement {
        use ast::Expr::*;
        match expr {
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
                if *i != 0 { t } else { f }
            )),
            Ident(i) => {
                assert!(i.kind.unwrap() == ast::Kind::int(),
                    "Identifier is not an integer!"
                );
                return ir::Statement::CJump(
                    Box::new(ir::Expr::Temp(i.id)),
                    t, f
                )
            },
            Float(_) => panic!("Floats found in conditional!"),
            _ => assert!(expr.kind().unwrap() == ast::Kind::int(),
                "Conditional is not an integer!"
            )
        }
        return ir::Statement::CJump(
            Box::new(self.expression(expr)),
            t, f
        )
    }
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
        let mut v = Vec::<ir::Expr>::new();
        for exp in &f.args {
            v.push(self.expression(exp));
        }
        return ir::Expr::Call(ir::Label { id: f.id }, v);
    }
    fn access(&mut self, a: &ast::AccessExpr) -> ir::Expr {
        let mut prod: u32 = 1;
        let t = ir::Expr::Temp(self.create_temp());
        let mut stmts = Vec::<ir::Statement>::new();
        for i in (0..a.offsets.len()).rev() {
            let mul = ir::Expr::BinOp(
                Box::new(ir::Expr::Const(
                    ir::Primitive::Int(
                        /* until I implement pointers correctly */ 
                        8 * prod as i32
                    )
                )),
                Operator::Mul,
                Box::new(self.expression(&a.offsets[i]))
            );
            let add = ir::Expr::BinOp(
                Box::new(t),
                Operator::Add,
                Box::new(mul)
            );
            stmts.push(ir::Statement::Move(
                Box::new(t),
                Box::new(add)
            ));
            prod *= a.sizes[i];
        }
        return ir::Expr::ESeq(
            Box::new(ir::Statement::Seq(stmts)),
            Box::new(ir::Expr::Mem(Box::new(t)))
        );
    }
    fn unary(&mut self, u: &ast::UnaryExpr) -> ir::Expr {
        use ast::UnaryOp::*;
        let op = match u.unary_op {
            Star    => ir::Operator::Star,
            Not     => ir::Operator::Not,
            Neg     => ir::Operator::Neg,
            Address => ir::Operator::Address,
        };
        return ir::Expr::UnOp(op,
            Box::new(self.expression(&u.expr))
        );
    }
    fn binary(&mut self, b: &ast::BinaryExpr) -> ir::Expr {
        use ast::BinaryOp::*;
        let mut op = match b.binary_op {
            Mul    => ir::Operator::Mul,
            Div    => ir::Operator::Div,
            Add    => ir::Operator::Add,
            Sub    => ir::Operator::Sub,
            Leq    => ir::Operator::Leq,
            Geq    => ir::Operator::Geq,
            Lt     => ir::Operator::Lt,
            Gt     => ir::Operator::Gt,
            Eq     => ir::Operator::Eq,
            Peq    => ir::Operator::Assign,
            Teq    => ir::Operator::Assign,
            Deq    => ir::Operator::Assign,
            Seq    => ir::Operator::Assign,
            Neq    => ir::Operator::Assign,
            Or     => ir::Operator::Or,
            And    => ir::Operator::And,
            Assign => ir::Operator::Assign,
        };
        if op != ir::Operator::Assign {
            return ir::Expr::BinOp(
                Box::new(self.expression(&b.left)),
                op,
                Box::new(self.expression(&b.right)),
            );
        }
        op = match b.binary_op {
            Peq => ir::Operator::Add,
            Teq => ir::Operator::Mul,
            Seq => ir::Operator::Sub,
            Deq => ir::Operator::Div,
            _   => ir::Operator::Xor,
        };
        let stmt = match b.binary_op {
            Peq | Teq | Seq | Deq => ir::Statement::Move(
                Box::new(ir::Expr::Temp(b.left.id())),
                Box::new(ir::Expr::BinOp(
                    Box::new(ir::Expr::Temp(b.left.id())),
                    op,
                    Box::new(self.expression(&b.right))
                ))
            ),
            Assign => ir::Statement::Move(
                Box::new(ir::Expr::Temp(b.left.id())),
                Box::new(self.expression(&b.right))
            ),
            _ => panic!("Oh no!")
        };
        return ir::Expr::ESeq(
            Box::new(stmt),
            Box::new(self.expression(&b.left))
        );
    }
    fn create_label(&mut self) -> ir::Label {
        self.nlabels += 1;
        return ir::Label { id: self.nlabels - 1 }
    }
    fn create_temp(&mut self) -> u32 {
        self.ids += 1;
        return self.ids - 1;
    }
}