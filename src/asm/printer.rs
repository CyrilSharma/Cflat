use super::asm::*;
pub struct Printer;
impl Printer {
    pub fn new() -> Self { Self {} }
    pub fn print(&mut self, instructions: &[AA]) {
        for ins in instructions {
            if matches!(ins, AA::Label(_)) {
                println!("\n{}", ins);
                continue;
            }
            println!("{}", ins);
        }
    }
    pub fn print_live(&mut self, ins: &[AA], live: &[Vec<Reg>]) {
        for (idx, i) in ins.iter().enumerate() {
            let rlist = live[idx].iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<String>>()
                .join(", ");
            if matches!(i, AA::Label(_)) { println!() }
            println!("{:<40} | {}", format!("{}", i), rlist);
        }
    }
}