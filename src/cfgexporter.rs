use crate::cfg::CFG;
use crate::ir::{Statement, self};
use bumpalo::Bump;

struct Exporter<'l> { arena: &'l mut Bump }
impl<'l> Exporter<'l> {
    fn new(arena: &'l mut Bump) -> Self {
        return Self { arena }
    }
    fn export(&mut self, cfg: CFG, order: Vec<usize>) -> Vec<&mut Statement> {
        let mut res = Vec::<&mut Statement>::new();
        for idx in order.clone() {
            let n = &cfg.nodes[idx];
            res.push(self.arena.alloc(Statement::Label(idx as u32)));
            res.extend(n.stmts);
            let last = res.pop().unwrap();
            let peek = || -> Option<usize> { 
                match idx + 1 == order.len() {
                    true => None,
                    false => Some(idx + 1)
                }
            };
            let mut jump = || -> Vec<&mut Statement> { match peek() {
                None => vec![last],
                Some(nxt) => match n.t.unwrap() == nxt {
                    true => vec![last],
                    false => vec![],
                }
            }};
            let mut cjump = |e: &mut &mut ir::Expr, flabel: &mut ir::Label| {
                *flabel = 0;
                if let Some(nxt) = peek() {
                    if n.t.unwrap() == nxt {
                        *e = self.arena.alloc(ir::Expr::UnOp(
                            ir::Operator::Not, *e
                        ));
                        return vec![last]
                    } else if n.f.unwrap() == nxt {
                        return vec![last]
                    }
                }
                vec![last, self.arena.alloc(
                    Jump(n.f.unwrap() as u32)
                )]
            };
            use Statement::*;
            res.extend(match &last {
                Jump(_) => jump(),
                CJump(mut e, _, mut f) => cjump(&mut e, &mut f),
                Return(_) => vec![last],
                Label(_) | Seq(_) => unreachable!(),
                _ => vec![last]
            });
        }
        return res;
    }
}