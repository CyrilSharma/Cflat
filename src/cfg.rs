use crate::ir::*;
#[derive(Clone)]
pub struct Node {
    pub stmts: Vec<Statement>,
    pub edges: Vec<usize>
}
pub struct CFG { 
    pub nodes: Vec<Node>,
    pub start: usize
}

impl CFG {
    fn export(&self, order: &Vec<usize>) -> Vec<Statement> {
        use Statement::*;
        for idx in order {
            match self.nodes[*idx].stmts.last().unwrap() {
                Expr(_) => todo!(),
                Move(_, _) => todo!(),
                Seq(_) => todo!(),
                Jump(_) => todo!(),
                CJump(_, _, _) => todo!(),
                Label(_) => todo!(),
                Return(_) => todo!(),
            }
        }
        todo!();
    }
}