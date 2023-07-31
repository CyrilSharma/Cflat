use crate::ir::{Statement, self};
#[derive(Clone)]
pub struct Node {
    pub stmts: Vec<Statement>,
    pub t: Option<usize>,
    pub f: Option<usize>
}
pub struct CFG { 
    pub nodes: Vec<Node>,
    pub start: usize
}

impl CFG {
    fn export(&self, order: &Vec<usize>) -> Vec<Statement> {
        let mut lcount = 0;
        let mut res = Vec::<Statement>::new();
        let mut iter = order.iter().cloned().peekable();
        while let Some(idx) = iter.next() {
            let n = &self.nodes[idx];
            res.push(Statement::Label(ir::Label{id: lcount}));
            res.extend(n.stmts.iter()
                .take(n.stmts.len() - 1)
                .cloned()
            );
            let last = n.stmts.last().unwrap();
            use Statement::*;
            match &last {
                Jump(_) => match iter.peek() {
                    None => res.push(Jump(ir::Label {
                        id: n.t.unwrap() as u32
                    })),
                    Some(nxt) => match n.t.unwrap() == *nxt {
                        true => res.push(Jump(ir::Label {
                            id: n.t.unwrap() as u32
                        })),
                        false => (),
                    }
                },
                CJump(e, _, _) => match iter.peek() {
                    None => {
                        res.push(CJump(
                            Box::new(*e.clone()),
                            ir::Label { id: n.t.unwrap() as u32 },
                            ir::Label { id: 0 }
                        ));
                        res.push(Jump(ir::Label {
                            id: n.f.unwrap() as u32
                        }));
                    }
                    Some(nxt) => {
                        if n.t.unwrap() == *nxt {
                            res.push(CJump(
                                Box::new(ir::Expr::UnOp(
                                    ir::Operator::Not,
                                    Box::new(*e.clone())
                                )),
                                ir::Label { id: n.t.unwrap() as u32 },
                                ir::Label { id: 0 }
                            ));
                        } else if n.f.unwrap() == *nxt {
                            res.push(CJump(
                                Box::new(*e.clone()),
                                ir::Label { id: n.t.unwrap() as u32 },
                                ir::Label { id: 0 }
                            ));
                        } else {
                            res.push(CJump(
                                Box::new(*e.clone()),
                                ir::Label { id: n.t.unwrap() as u32 },
                                ir::Label { id: 0 }
                            ));
                            res.push(Jump(ir::Label {
                                id: n.f.unwrap() as u32
                            }));
                        }
                    }
                },
                Return(_) => res.push(last.clone()),
                Label(_) | Seq(_) => unreachable!(),
                _ => match iter.peek() {
                    None => res.push(last.clone()),
                    Some(nxt) => match n.t.unwrap() == *nxt {
                        true => res.push(last.clone()),
                        false => {
                            res.push(last.clone());
                            res.push(Jump(ir::Label {
                                id: n.t.unwrap() as u32
                            }));
                        },
                    }
                }
            };
            lcount += 1;
        }
        return res;
    }
}