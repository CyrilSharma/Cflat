use super::asm::*;
pub struct Printer;
impl Printer {
    // Removes no-ops.
    pub fn print(instructions: &[AA]) {
        for ins in instructions {
            match ins {
                AA::Label(_) => println!("\n{}", ins),
                AA::Mov2(d, s) if d == s => (),
                _ => println!("{}", ins)
            }
        }
    }

    pub fn print_live(instructions: &[AA], live: &[Vec<Reg>]) {
        for (ins_idx, ins) in instructions.iter().enumerate() {
            let mut ind = 0;
            let mut rlists: Vec<String> = Vec::new();
            while ind < live[ins_idx].len() {
                let mut s = String::new();
                while ind < live[ins_idx].len() {
                    let add = format!(
                        "{:<10}",
                        format!("{}", live[ins_idx][ind])
                    );
                    if s.len() + add.len() > 40 { break }
                    s += &add;
                    ind += 1;
                }
                rlists.push(s);
                ind += 1;
            }
            if matches!(ins, AA::Label(_)) { println!() }
            print!("{:<40}", format!("{}", ins));
            if rlists.len() == 0 { println!() }
            for (j, s) in rlists.iter().enumerate() {
                if j == 0 { println!(" | {}", s); } 
                else { println!("{:<40} * {}", "", s); }
            }
        }
    }
}