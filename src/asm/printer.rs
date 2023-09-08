use super::asm::*;
pub struct Printer;
impl Printer {
    // Removes no-ops.
    pub fn print(instructions: &[AA]) {
        println!(".global __start");
        for ins in instructions {
            match ins {
                AA::BB(_) => (),
                AA::Label(_) => println!("\n{}", ins),
                AA::Mov2(d, s) if d == s => (),
                _ => println!("{}", ins)
            }
        }
        println!("\n\n\n");
    }

    pub fn print_raw(instructions: &[AA]) {
        println!(".global __start");
        for ins in instructions {
            match ins {
                AA::BB(_) => println!("\n{}", ins),
                AA::Label(_) => println!("{}", ins),
                _ => println!("{}", ins)
            }
        }
        println!("\n\n\n");
    }
}