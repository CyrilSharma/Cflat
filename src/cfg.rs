use std::collections::{HashMap, HashSet};
use crate::ir::{self, *};

#[derive(Clone)]
pub struct Node {
    pub stmts: Vec<Statement>,
    pub edges: Vec<usize>
}
pub struct CFG { pub nodes: Vec<Node> }
pub struct CfgBuilder {
    idx: usize, // the current statement
    nid: usize, // the current block
    link: bool, // whether prev & cur should be connectd.
    lookup: HashMap<u32, usize>,
    nodes: Vec<Node>
}
// TODO: Organize, use LinkedLists for Nodes,
// and prune duplicate connections with Hash tables.
impl CfgBuilder {
    fn new() -> CfgBuilder  { 
        CfgBuilder {
            idx: 0,
            nid: 0,
            link: true,
            lookup: HashMap::new(),
            nodes: Vec::new()
        }
    }
    fn build(&mut self, stmts: &Vec<Statement>) -> CFG {
        use Statement::*;
        for stmt in stmts {
            match stmt {
                Expr(e)          => self.expr(&e),
                Move(d, s)       => self._move(&d, &s),
                Jump(l)          => self.jump(&l),
                CJump(e, l1, l2) => self.cjump(&e, &l1, &l2),
                Return(r)        => self._return(&r),
                Label(l)         => self.label(&l),
                _ => unreachable!()
            }
        }
        // Deduplicate Edges.
        for node in &self.nodes {
            node.edges = node.edges.into_iter()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
        }
        return CFG { nodes: std::mem::take(&mut self.nodes) }
    }
    fn expr(&mut self, e: &Expr) {
        if let ir::Expr::Call(f, _) = e {
            let id1 = self.get(f.id);
            self.nodes[self.nid].edges.push(id1);
            self.nodes[id1].edges.push(self.nid);
        }
        self.nodes[self.nid].stmts.push(
            Statement::Expr(Box::new(*e))
        );
        self.link = true;
    }
    fn _move(&mut self, d: &Expr, s: &Expr) {
        self.nodes[self.nid].stmts.push(
            Statement::Move(
                Box::new(*d),
                Box::new(*s)
            )
        );
        self.link = true;
    }
    fn jump(&mut self, l: &Label) {
        self.nodes[self.nid].stmts.push(
            Statement::Jump(*l)
        );
        let id = self.get(l.id);
        self.nodes[self.nid].edges.push(id);
        self.link = false;
        self.nid = self.create_node();
    }
    fn cjump(&mut self, e: &Expr, l1: &Label, l2: &Label) {
        self.nodes[self.nid].stmts.push(
            Statement::CJump(
                Box::new(*e),
                *l1, *l2
            )
        );
        let id1 = self.get(l1.id);
        self.nodes[self.nid].edges.push(id1);
        let id2 = self.get(l2.id);
        self.nodes[self.nid].edges.push(id2);
        self.link = false;
        self.nid = self.create_node();
    }
    fn _return(&mut self, o: &Option<Box<Expr>>) {
        self.nodes[self.nid].stmts.push(
            Statement::Return(
                match o {
                    None => None,
                    Some(e) => Some(Box::new(**e))
                }
            )
        );
        self.link = false;
        self.nid = self.create_node();
    }
    fn label(&mut self, l: &Label) {
        let old = self.nid;
        let mut removed = false;
        if self.nodes[old].stmts.len() == 0 {
            self.nodes.pop();
            removed = true;
        }
        self.nid = self.get(l.id);
        self.nodes[self.nid].stmts.push(
            Statement::Label(*l)
        );
        if !removed && self.link {
            self.nodes[old].edges.push(self.nid);
        }
    }
    fn get(&mut self, i: u32) -> usize {
        match self.lookup.get(&i) {
            None => {
                self.lookup.insert(i, self.nodes.len());
                return self.create_node();
            }
            Some(id) => return *id
        };
    }
    fn create_node(&mut self) -> usize {
        self.nodes.push(Node { 
            stmts: Vec::new(), 
            edges: Vec::new() 
        });
        return self.nodes.len() - 1;
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use std::fs;
    use std::path::Path;
    use crate::parser::moduleParser;
    use crate::semantic::Semantic;
    //use crate::astprinter;
    //use crate::irprinter;
    use crate::irtranslator::Translator;
    use crate::irreducer::Reducer;
    use crate::cfgprinter;

    #[test]
    fn visualize() {
        let mut i = 0;
        let dir = "tests/data";
        while Path::new(&format!("{dir}/input{i}.c")).exists() {
            let filepath = &format!("{dir}/input{i}.c");
            let input = fs::read_to_string(filepath).expect("File not found!");
            let mut m = moduleParser::new().parse(&input).expect("Parse Error!");
            let mut semantic = Semantic::new();
            semantic.analyze(&mut m);
            let ir  = Translator::new().translate(&mut m);
            let lir = Reducer::new(semantic.nid()).reduce(&ir);
            let cfg = CfgBuilder::new().build(&lir);
            println!("{}", &format!("FILE: {filepath}"));
            cfgprinter::Printer::new().print(&cfg.nodes);
            //irprinter::Printer::new().print(&lir);
            println!("\n\n\n\n\n");
            i += 1;
        }   
    }
}