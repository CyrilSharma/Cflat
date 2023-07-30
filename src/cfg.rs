use std::collections::HashMap;
use crate::ir::{self, *};

#[derive(Clone)]
pub struct Node {
    pub stmts: Vec<Statement>,
    pub edges: Vec<usize>
}
pub struct CFG { pub nodes: Vec<Node> }
pub struct CfgBuilder {
    nodes: Vec<Node>,
    lookup: HashMap<u32, usize>
}
impl CfgBuilder {
    fn new() -> CfgBuilder  { 
        CfgBuilder {
            nodes: Vec::new(),
            lookup: HashMap::new()
        } 
    }
    fn build(&mut self, stmts: &Vec<Statement>) -> CFG {
        use Statement::*;
        let mut nid;
        let mut idx: usize = 0;
        let mut shift_control = true;
        while idx < stmts.len() {
            nid = self.nodes.len();
            self.nodes.push(Node { 
                stmts: Vec::new(),
                edges: Vec::new()
            });
            while idx < stmts.len() {
                idx += 1; // counter increases even on break.
                match stmts[idx - 1] {
                    Expr(ref e) => {
                        if let ir::Expr::Call(f, _) = **e {
                            let id1 = self.find(f.id);
                            self.nodes[nid].edges.push(id1);
                            self.nodes[id1].edges.push(nid);
                        }
                        self.nodes[nid].stmts.push(
                            stmts[idx-1].clone()
                        );
                        shift_control = true;
                    },
                    Move(_, _) => {
                        self.nodes[nid].stmts.push(
                            stmts[idx-1].clone()
                        );
                        shift_control = true;
                    },
                    Jump(l) => {
                        self.nodes[nid].stmts.push(
                            stmts[idx-1].clone()
                        );
                        let id = self.find(l.id);
                        self.nodes[nid].edges.push(id);
                        shift_control = false;
                        break;
                    },
                    CJump(_, l1, l2) => {
                        self.nodes[nid].stmts.push(
                            stmts[idx-1].clone()
                        );
                        let id1 = self.find(l1.id);
                        self.nodes[nid].edges.push(id1);
                        let id2 = self.find(l2.id);
                        self.nodes[nid].edges.push(id2);
                        shift_control = false;
                        break;
                    },
                    Return(_) => {
                        self.nodes[nid].stmts.push(
                            stmts[idx-1].clone()
                        );
                        shift_control = false;
                        break;
                    },
                    Label(l) => {
                        /* remove empty nodes. */
                        let old = nid;
                        let mut removed = false;
                        if self.nodes[old].stmts.len() == 0 {
                            self.nodes.pop();
                            removed = true;
                        }
                        nid = self.find(l.id);
                        self.nodes[nid].stmts.push(
                            stmts[idx-1].clone()
                        );
                        if !removed && shift_control {
                            self.nodes[old].edges.push(nid);
                        }
                    }
                    _ => unreachable!()
                }
            }
        }
        return CFG { nodes: self.nodes.clone() }
    }
    fn find(&mut self, i: u32) -> usize {
        return match self.lookup.get(&i) {
            None => {
                self.lookup.insert(i, self.nodes.len());
                self.nodes.push(Node { stmts: Vec::new(), edges: Vec::new() });
                self.nodes.len() - 1
            }
            Some(id) => *id
        };
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
    use crate::irprinter;
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