use super::cfg::*;
pub struct Printer { count: u32 }
/* note that this only works for LIR CFG */
impl Printer {
    pub fn print(cfg: &CFG) {
        let mut printer = Self { count: 0 };
        println!("digraph CFG {{");
        println!(r#"  node [shape=box, fontname="Helvetica", fontsize=12]"#);
        let idx = cfg.start;
        println!("{}", format!(
            r#"  node{} [label="Start {}"]"#,
            cfg.nodes.len(), idx as u32
        ));
        printer.add_edge(
            (cfg.nodes.len()) as u32, 
            idx as u32
        );
        for n in &cfg.nodes {
            let asm = cfg.asm[n.idx];
            printer.label(format!("{}", asm));
            if let Some(e) = n.t { 
                printer.add_edge(printer.count, e as u32) 
            }
            if let Some(e) = n.f { 
                printer.add_edge(printer.count, e as u32) 
            }
            printer.count += 1;
        }
        println!("}}\n");
    }
    fn add_edge(&self, i: u32, j: u32) {
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