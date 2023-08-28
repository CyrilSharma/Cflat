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
}