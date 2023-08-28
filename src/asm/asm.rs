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

#[derive(Copy, Clone)]
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