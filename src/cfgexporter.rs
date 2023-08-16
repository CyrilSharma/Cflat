use crate::ir::{self, *};
use crate::cfg::CFG;
fn export(cfg: CFG, order: Vec<usize>) -> Vec<Box<Statement>> {
    let mut res = Vec::<Box<Statement>>::new();
    for ind in 0..order.len() {
        let idx = order[ind];
        let n = &cfg.nodes[idx];
        res.push(Box::new(Statement::Label(idx as u32)));
        res.extend(n.stmts);
        let last = res.pop().unwrap();
        let peek = || { 
            match ind + 1 == order.len() {
                false => Some(order[ind+1]),
                true  => None
            }
        };
        let mut jump = || {
            let pk = peek();
            match n.t == pk {
                false => vec![last],
                true  => vec![],
            };
            vec![last]
        };
        let mut cjump = |
            e: &Box<ir::Expr>,
            t: &mut ir::Label,
            f: &mut ir::Label | {
            let pk = peek();
            if n.t == pk {
                use ir::Expr::UnOp;
                use ir::Operator::Not;
                *e = Box::new(UnOp(Not, *e));
                std::mem::swap(t, f);
                return vec![last]
            } else if n.f == pk {
                return vec![last]
            }
            let i = n.f.unwrap() as u32;
            let j = Box::new(Jump(i));
            vec![last, j]
        };
        use Statement::*;
        res.extend(match *last {
            CJump(mut e, mut t, mut f)
                      => cjump(&mut e, &mut t, &mut f),
            Jump(_)   => jump(),
            Return(_) => vec![last],
            _         => unreachable!(),
        });
    }
    return res;
}