use super::ir::{self, *};
use super::cfg::CFG;
const INVALID: u32 = 1e9 as u32;
pub fn export(mut cfg: CFG, order: Vec<usize>) -> Vec<Box<Statement>> {
    let mut res = Vec::<Box<Statement>>::new();
    for ind in 0..order.len() {
        let idx = order[ind];
        let n = &mut cfg.nodes[idx];
        let mut cur = std::mem::take(&mut n.stmts);
        let last = cur.pop().unwrap();
        let peek = || { 
            match ind + 1 == order.len() {
                false => Some(order[ind+1]),
                true  => None
            }
        };
        use Statement::*;
        cur.extend(match *last {
            // I don't know how to avoid this extra allocation.
            CJump(e, _, _) => {
                let pk = peek();
                let nt = n.t.unwrap() as u32;
                let nf = n.f.unwrap() as u32;
                if n.t == pk {
                    use ir::Expr::UnOp;
                    use ir::Operator::Not;
                    let ne = Box::new(UnOp(Not, e));
                    vec![Box::new(CJump(ne, nf, INVALID))]
                } else if n.f == pk {
                    vec![Box::new(CJump(e,  nt, INVALID))]
                } else {
                    let i = n.f.unwrap() as u32;
                    let j = Box::new(Jump(i));
                    vec![Box::new(CJump(e, nt, INVALID)), j]
                }
            },
            Jump(_)   => {
                let pk = peek();
                match n.t == pk {
                    false => vec![last],
                    true  => vec![],
                }
            },
            _ => {
                let pk = peek();
                if n.f == pk || matches!(n.f, None) {
                    vec![last]
                } else {
                    let nf = n.f.unwrap() as u32;
                    let j = Box::new(Jump(nf));
                    vec![last, j]
                }
            }
        });
        res.extend(cur);
    }
    return res;
}