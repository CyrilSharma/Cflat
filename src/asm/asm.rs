use core::fmt;
use std::fmt::Display;

pub type Label = u32;
pub type Const = usize;
// Presume everything costs the same.

#[derive(Copy, Clone)]
pub enum AA {
    Label(Label),
    Mov1(Reg, Const),
    Mov2(Reg, Reg),
    Add1(Reg, Reg, Const),
    Add2(Reg, Reg, Reg),
    Sub1(Reg, Reg, Const),
    Sub2(Reg, Reg, Reg),
    Neg1(Reg, Const),           // Xd = -Xs
    Neg2(Reg, Reg),             // Xd = -Xs
    SMAddL(Reg, Reg, Reg, Reg), // Xd = Xa + (Wn × Wm)
    SMNegL(Reg, Reg, Reg),      // Xd = - (Wn × Wm)
    SMSubL(Reg, Reg, Reg, Reg), // Xd = Xa − (Wn × Wm)
    SMulL(Reg, Reg, Reg),
    SDiv(Reg, Reg, Reg),
    And1(Reg, Reg, Const),
    And2(Reg, Reg, Reg),
    Or1(Reg, Reg, Const),
    Or2(Reg, Reg, Reg),
    Mvn1(Reg, Const),           // Xd = ~Xs
    Mvn2(Reg, Reg),             // Xd = ~Xs
    B(Label),
    BL(Label),                  // R30 = SP
    CBZ(Label),
    CBNZ(Label),
    CMP1(Reg, Const),
    CMP2(Reg, Reg),
    CSET(Reg, CC),
    LDR1(Reg, Reg, Const),
    LDR2(Reg, Reg),
    STR1(Reg, Reg, Const),
    STR2(Reg, Reg),
    Ret
}
impl Display for AA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AA::*;
        let res = match self {
            Label(l)           => format!("l{}: ", l),
            Mov1(d, s)         => format!("mov {}, #{}", d, s),
            Mov2(d, s)         => format!("mov {}, {}", d, s),
            Add1(d, l, r)      => format!("add {}, {}, #{}", d, l, r),
            Add2(d, l, r)      => format!("add {}, {}, {}", d, l, r),
            Sub1(d, l, r)      => format!("sub {}, {}, #{}", d, l, r),
            Sub2(d, l, r)      => format!("sub {}, {}, {}", d, l, r),
            Neg1(d, s)         => format!("neg {}, #{}", d, s),
            Neg2(d, s)         => format!("neg {}, {}", d, s),
            SMAddL(d, l, m, r) => format!("smaddl {}, {}, {}, {}", d, l, m, r),
            SMNegL(d, l, r)    => format!("smnegl {}, {}, {}", d, l, r),
            SMSubL(d, l, m, r) => format!("smsubl {}, {}, {}, {}", d, l, m, r),
            SMulL(d, l, r)     => format!("smull {}, {}, {}", d, l, r),
            SDiv(d, l, r)      => format!("sdiv {}, {}, {}", d, l, r),
            And1(d, l, r)      => format!("and {}, {}, #{}", d, l, r),
            And2(d, l, r)      => format!("and {}, {}, {}", d, l, r),
            Or1(d, l, r)       => format!("or {}, {}, #{}", d, l, r),
            Or2(d, l, r)       => format!("or {}, {}, {}", d, l, r),
            Mvn1(d, s)         => format!("mvn {}, #{}", d, s),
            Mvn2(d, s)         => format!("mvn {}, {}", d, s),
            B(l)               => format!("b l{}", l),
            BL(l)              => format!("bl l{}", l),
            CBZ(l)             => format!("cbz l{}", l),
            CBNZ(l)            => format!("cbnz l{}", l),
            CMP1(d, s)         => format!("cmp {}, #{}", d, s),
            CMP2(d, s)         => format!("cmp {}, {}", d, s),
            CSET(d, s)         => format!("cset {}, {}", d, s),
            LDR1(d, l, r)      => format!("ldr {}, [{}, #{}]", d, l, r),
            LDR2(d, s)         => format!("ldr {}, [{}]", d, s),
            STR1(d, l, r)      => format!("str {}, [{}, #{}]", d, l, r),
            STR2(d, s)         => format!("str {}, [{}]", d, s),
            Ret                => format!("ret"),
        };
        write!(f, "{res}")
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Reg {
    // Args && Return Values
    // R0, R1, R2, R3, R4, R5, R6, R7,
    // Indirect Result
    // R8,
    // Temporary
    // R9, R10, R11, R12, R13, R14, R15,
    // ???
    // R18,
    // Temporary (must be preserved)
    // R19, R20, R21, R22, R23, R24, R25,
    // R26, R27, R28,
    // Frame Pointer (must be preserved)
    // R29,
    // Return Address
    // R30,
    R(u8),
    // Stack Pointer
    SP,
    // Zero
    RZR,
    // Program Counter
    PC,
    // Virtual Registers
    ID(u32)
}
impl Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Reg::*;
        match self {
            R(i)  => write!(f, "R{}", i),
            SP    => write!(f, "SP"),
            RZR   => write!(f, "RZR"),
            PC    => write!(f, "PC"),
            ID(i) => write!(f, "ID({})", i),
        }
    }
}

#[derive(Copy, Clone)]
pub enum CC {
    EQ,
    NE,
    GE,
    LT,
    GT,
    LE
}
impl Display for CC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CC::*;
        match self {
            EQ => write!(f, "EQ"),
            NE => write!(f, "NE"),
            GE => write!(f, "GE"),
            LT => write!(f, "LT"),
            GT => write!(f, "GT"),
            LE => write!(f, "LE"),
        }
    }
}