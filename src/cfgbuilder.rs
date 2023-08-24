use crate::ir::*;
use crate::cfg::*;
use crate::registry::Registry;
pub fn build(r: &Registry, stmts: Vec<Box<Statement>>) -> CFG {
    let mut nodes  = vec![Node::new(); r.nlabels as usize];
    let starts = (0..(r.nfuncs as usize)).collect();
    let mut iter = stmts.into_iter().peekable();
    let mut cur = nodes.len();
    while let Some(stmt) = iter.next() {
        use Statement::*;
        match *stmt {
            Seq(_)               => unreachable!(),
            Jump(l)              => {
                nodes[cur].stmts.push(stmt);
                nodes[cur].t = Some(l as usize);
                let Some(pk) = iter.peek() else { continue };
                if matches!(**pk, Label(_)) { continue };
                nodes.push(Node::new());
                cur = nodes.len() - 1;
            },
            CJump(_, l1, l2)     => {
                nodes[cur].stmts.push(stmt);
                nodes[cur].t = Some(l1 as usize);
                if l2 != INVALID {
                    nodes[cur].f = Some(l2 as usize);
                }
                let Some(pk) = iter.peek() else { continue };
                if let Label(l) = **pk { 
                    nodes[cur].f = Some(l as usize);
                } else {
                    nodes[cur].f = Some(nodes.len());
                    nodes.push(Node::new());
                    cur = nodes.len() - 1;
                }
            },
            Label(l)             => {
                cur = l as usize;
                nodes[cur].stmts.push(stmt);
            }
            _                    =>	{
                nodes[cur].stmts.push(stmt);
            },
        }
    }
    return CFG { nodes, starts }
}