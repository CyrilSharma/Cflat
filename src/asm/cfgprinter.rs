use super::cfg::*;
pub struct Printer { count: u32 }
/* note that this only works for LIR CFG */
impl Printer {
    pub fn new() -> Self { Self{count: 0} }
    pub fn print(&mut self, cfg: &CFG) {
        println!("digraph CFG {{");
        println!(r#"  node [shape=box, fontname="Helvetica", fontsize=12]"#);
        let idx = cfg.start;
        println!("{}", format!(
            r#"  node{} [label="Start {}"]"#,
            cfg.nodes.len(), idx as u32
        ));
        self.add_edge(
            (cfg.nodes.len()) as u32, 
            idx as u32
        );
        for n in &cfg.nodes {
            self.node(n);
            self.count += 1;
        }
        println!("}}\n");
    }
    fn node(&mut self, n: &Node) {
        self.label(format!("{}", n.asm));
        if let Some(e) = n.t { 
            self.add_edge(self.count, e as u32) 
        }
        if let Some(e) = n.f { 
            self.add_edge(self.count, e as u32) 
        }
    }
    fn add_edge(&mut self, i: u32, j: u32) {
        println!("    node{} -> node{};", i, j)
    }
    fn label(&self, s: String) {
        let stmt = format!(
            "          <tr><td align=\"center\">\"{}\"</td></tr>\n",
            s
        );
        println!("{}",
            format!( "  node{} [label=<\n", self.count) +
            &format!("    <table border=\"0\" cellborder=\"1\" cellspacing=\"0\" cellpadding=\"6\">\n") + 
            &format!("      <tr><td align=\"center\" bgcolor=\"lightblue\"><font color=\"black\"><b>Node {}</b></font></td></tr>\n", self.count) + 
            
            &format!("{}", stmt) + 
            
            &format!("    </table>\n") + 
            &format!("  >]") 
        );
    }
}