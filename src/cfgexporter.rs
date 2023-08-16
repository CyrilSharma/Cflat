use crate::ir::{self, *};
use crate::cfg::CFG;
pub fn export(mut cfg: CFG, order: Vec<usize>) -> Vec<Box<Statement>> {
    let mut res = Vec::<Box<Statement>>::new();
    for ind in 0..order.len() {
        let idx = order[ind];
        let n = &mut cfg.nodes[idx];
        res.push(Box::new(Statement::Label(idx as u32)));
        res.extend(std::mem::take(&mut n.stmts));
        let last = res.pop().unwrap();
        let peek = || { 
            match ind + 1 == order.len() {
                false => Some(order[ind+1]),
                true  => None
            }
        };
        use Statement::*;
        res.extend(match *last {
            // I don't know how to avoid this extra allocation.
            CJump(e, t, f) => {
                let pk = peek();
                if n.t == pk {
                    use ir::Expr::UnOp;
                    use ir::Operator::Not;
                    let e2 = Box::new(UnOp(Not, e));
                    vec![Box::new(CJump(e2, f, t))]
                } else if n.f == pk {
                    vec![Box::new(CJump(e, t, f))]
                } else {
                    let i = n.f.unwrap() as u32;
                    let j = Box::new(Jump(i));
                    vec![Box::new(CJump(e, t, f)), j]
                }
            },
            Jump(_)   => {
                let pk = peek();
                match n.t == pk {
                    false => vec![last],
                    true  => vec![],
                }
            },
            _ => vec![last]
        });
    }
    return res;
}