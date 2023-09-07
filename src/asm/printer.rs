use super::asm::*;
pub struct Printer;
impl Printer {
    // Removes no-ops.
    pub fn print(instructions: &[AA]) {
        for ins in instructions {
            match ins {
                AA::BB(_) => (),
                AA::Label(_) => println!("\n{}", ins),
                AA::Mov2(d, s) if d == s => (),
                _ => println!("{}", ins)
            }
        }
    }

    pub fn print_raw(instructions: &[AA]) {
        for ins in instructions {
            match ins {
                AA::Label(_) => println!("\n{}", ins),
                _ => println!("{}", ins)
            }
        }
    }
}