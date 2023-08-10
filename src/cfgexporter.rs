use crate::cfg::CFG;
use crate::ir::{Statement, self};
use bumpalo::Bump;

struct Exporter<'l> { arena: &'l mut Bump }
impl<'l> Exporter<'l> {
    fn new(arena: &'l mut Bump) -> Self {
        return Self { arena }
    }
    fn export(&self, cfg: CFG, order: &Vec<usize>) -> Vec<Statement> {
        let mut res = Vec::<Statement>::new();
        for idx in order.clone() {
            let n = &cfg.nodes[idx];
            res.push(Statement::Label(idx as u32));
            res.extend(n.stmts.iter()
                .take(n.stmts.len() - 1)
                .cloned()
            );
            let last = n.stmts.last().unwrap();
            let peek = || -> Option<usize> { 
                match idx + 1 == order.len() {
                    true => None,
                    false => Some(idx + 1)
                }
            };
            let jump = || -> Vec<Statement> { match peek() {
                None => vec![Jump(n.t.unwrap() as u32)],
                Some(nxt) => match n.t.unwrap() == nxt {
                    true => vec![Jump(n.t.unwrap() as u32)],
                    false => vec![],
                }
            }};
            let cjump = |e: &ir::Expr| { match peek() {
                None => vec![
                    CJump(
                        self.arena.alloc(e.clone()),
                        n.t.unwrap() as u32,
                        0
                    ),
                    Jump(n.f.unwrap() as u32)
                ],
                Some(nxt) => match (n.t.unwrap() == nxt,
                                    n.f.unwrap() == nxt) {
                    (true, true) => unreachable!(),
                    (true, false) => vec![CJump(
                        self.arena.alloc(ir::Expr::UnOp(
                            ir::Operator::Not,
                            self.arena.alloc(e.clone())
                        )),
                        n.t.unwrap() as u32,
                        0
                    )],
                    (false, true) => vec![CJump(
                        self.arena.alloc(e.clone()),
                        n.t.unwrap() as u32,
                        0
                    )],
                    (false, false) => vec![
                        CJump(
                            self.arena.alloc(e.clone()),
                            n.t.unwrap() as u32,
                            0
                        ),
                        Jump(n.f.unwrap() as u32)
                    ]
                }
            }};
            let notransfer = || -> Vec<Statement> { match peek() {
                None => vec![last.clone()],
                Some(nxt) => match n.t.unwrap() == nxt {
                    true => vec![last.clone()],
                    false => vec![
                        last.clone(),
                        Jump(n.t.unwrap() as u32)
                    ],
                }
            }};
            use Statement::*;
            res.extend(match &last {
                Jump(_) => jump(),
                CJump(e, _, _) => cjump(e),
                Return(_) => vec![last.clone()],
                Label(_) | Seq(_) => unreachable!(),
                _ => notransfer()
            });
        }
        return res;
    }
}