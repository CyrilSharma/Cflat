pub type Label = u32;
pub enum AA {
    Mov(Reg, Reg),
    Add(Reg, Reg, Reg),
    Sub(Reg, Reg, Reg),
    MAdd(Reg, Reg, Reg),
    MNeg(Reg, Reg),
    MSub(Reg, Reg, Reg),
    Mul(Reg, Reg),
    SDiv(Reg, Reg),
    Dec(Reg),
    And(Reg),
    Or(Reg),
    Xor(Reg),
    Not(Reg),
    B(Label),
    Bcc(Label),
    BL(Label),
    CBZ(Label),
    RET(Label),
    Jne(Label),
    Jl(Label),
    Jle(Label),
    Jg(Label),
    Jge(Label),
    Push(Reg),
    Pop(Ref),
    Call(Label),
    Ret
}

pub enum Reg {
    // Args && Return Values
    X0, X1, X2, X3, X4, X5, X6, X7,
    // Indirect Result
    X8,
    // Temporary
    X9, X10, X11, X12, X13, X14, X15,
    // ???
    X18,
    // Temporary (must be preserved)
    X19, X20, X21, X22, X23, X24, X25,
    X26, X27, X28,
    // Frame Pointer (must be preserved)
    X29,
    // Return Address
    X30,
    // Stack Pointer
    SP,
    // Zero
    XZR,
    // Program Counter
    PC
}