use super::asm::*;
struct Allocator;
impl Allocator {
    pub fn allocate(asm: Vec<AA>, live: Vec<Vec<Reg>>) {
        let mut graph: Vec<Vec<Reg>> = Vec::new();
        for entry in live {
            for i in 0..entry.len() {
                let s = entry[i];
                for j in (i+1)..entry.len() {
                    let d = entry[j];
                    graph[s].push(d); 
                    graph[d].push(s);
                }
            }
        }
        todo!()
    }
    pub fn build() {}
}