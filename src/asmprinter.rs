use crate::asm::*;
pub struct Printer;
impl Printer {
    pub fn new() -> Self { Self {} }
    pub fn print(&mut self, instructions: &[AA]) {
        for ins in instructions {
            self.instruction(ins);
        }
    }
    fn instruction(&mut self, ins: &AA) {
        use AA::*;
        match ins {
            Label(l)           => println!("\nl{}: ", l),
            Mov1(d, s)         => println!("mov {}, #{}", d, s),
            Mov2(d, s)         => println!("mov {}, {}", d, s),
            Add1(d, l, r)      => println!("add {}, {}, #{}", d, l, r),
            Add2(d, l, r)      => println!("add {}, {}, {}", d, l, r),
            Sub1(d, l, r)      => println!("sub {}, {}, #{}", d, l, r),
            Sub2(d, l, r)      => println!("sub {}, {}, {}", d, l, r),
            Neg1(d, s)         => println!("neg {}, #{}", d, s),
            Neg2(d, s)         => println!("neg {}, {}", d, s),
            SMAddL(d, l, m, r) => println!("smaddl {}, {}, {}, {}", d, l, m, r),
            SMNegL(d, l, r)    => println!("smnegl {}, {}, {}", d, l, r),
            SMSubL(d, l, m, r) => println!("smsubl {}, {}, {}, {}", d, l, m, r),
            SMulL(d, l, r)     => println!("smull {}, {}, {}", d, l, r),
            SDiv(d, l, r)      => println!("sdiv {}, {}, {}", d, l, r),
            And1(d, l, r)      => println!("and {}, {}, #{}", d, l, r),
            And2(d, l, r)      => println!("and {}, {}, {}", d, l, r),
            Or1(d, l, r)       => println!("or {}, {}, #{}", d, l, r),
            Or2(d, l, r)       => println!("or {}, {}, {}", d, l, r),
            Mvn1(d, s)         => println!("mvn {}, #{}", d, s),
            Mvn2(d, s)         => println!("mvn {}, {}", d, s),
            B(l)               => println!("b l{}", l),
            BL(l)              => println!("bl l{}", l),
            CBZ(l)             => println!("cbz l{}", l),
            CBNZ(l)            => println!("cbnz l{}", l),
            CMP1(d, s)         => println!("cmp {}, #{}", d, s),
            CMP2(d, s)         => println!("cmp {}, {}", d, s),
            CSET(d, s)         => println!("cset {}, {}", d, s),
            LDR1(d, l, r)      => println!("ldr {}, [{}, #{}]", d, l, r),
            LDR2(d, s)         => println!("ldr {}, [{}]", d, s),
            STR1(d, l, r)      => println!("str {}, [{}, #{}]", d, l, r),
            STR2(d, s)         => println!("str {}, [{}]", d, s),
            Ret                => println!("ret"),
        }
    }
}