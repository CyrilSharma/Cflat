use core::fmt;
use std::fmt::Display;
use std::cmp::Ordering;

pub type Label = u32;
pub const GPRS: usize = 32;
// Presume everything costs the same.

#[derive(Clone)]
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
    Ret,

    BB(Vec<Reg>),              // Pseudo-OP
}
impl AA {
    #[allow(unused_variables)]
    pub fn defuse(&self) -> (Vec<Reg>, Vec<Reg>) {
        use AA::*;
        use Reg::*;
        return match self.clone() {
            Label(l)           => (vec![],    vec![]),
            Mov1(d, s)         => (vec![d],   vec![]),
            Mov2(d, s)         => (vec![d],   vec![s]),
            Add1(d, l, r)      => (vec![d],   vec![l]),
            Add2(d, l, r)      => (vec![d],   vec![l, r]),
            Sub1(d, l, r)      => (vec![d],   vec![l]),
            Sub2(d, l, r)      => (vec![d],   vec![l, r]),
            Neg1(d, s)         => (vec![d],   vec![]),
            Neg2(d, s)         => (vec![d],   vec![s]),
            SMAddL(d, l, m, r) => (vec![d],   vec![l, m, r]),
            SMNegL(d, l, r)    => (vec![d],   vec![l, r]),
            SMSubL(d, l, m, r) => (vec![d],   vec![l, r]),
            SMulL(d, l, r)     => (vec![d],   vec![l, r]),
            SDiv(d, l, r)      => (vec![d],   vec![l, r]),
            And1(d, l, r)      => (vec![d],   vec![l]),
            And2(d, l, r)      => (vec![d],   vec![l, r]),
            Or1(d, l, r)       => (vec![d],   vec![l]),
            Or2(d, l, r)       => (vec![d],   vec![l, r]),
            Mvn1(d, s)         => (vec![d],   vec![]),
            Mvn2(d, s)         => (vec![d],   vec![s]),
            B(l)               => (vec![],    vec![]),
            BL(l)              => (vec![],    vec![]),
            CBZ(l)             => (vec![],    vec![]),
            CBNZ(l)            => (vec![],    vec![]),
            CMP1(d, s)         => (vec![d],   vec![]),
            CMP2(d, s)         => (vec![d],   vec![s]),
            CSET(d, s)         => (vec![d],   vec![]),
            LDR1(d, l, r)      => (vec![d],   vec![l]),
            LDR2(d, s)         => (vec![d],   vec![s]),
            STR1(d, l, r)      => (vec![d],   vec![l]),
            STR2(d, s)         => (vec![d],   vec![s]),
            Ret                => (vec![SP],  vec![R(29)]),
            BB(v)              => (v.clone(), vec![])
        };
    }
}
impl Display for AA {
    #[allow(unused_variables)]
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
            BB(v)              => {
                let res = v.iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Basic Block {}", res)
            }
        };
        write!(f, "{res}")
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Reg {
    R(u8),
    SP,
    RZR,
    PC,
    // Virtual Registers
    ID(u32)
}
impl Reg {
    pub fn index(&self) -> usize {
        use Reg::*;
        // Only 30 other registers.
        match self {
            R(i)  => *i as usize,
            SP    => GPRS - 3,
            RZR   => GPRS - 2,
            PC    => GPRS - 1,
            ID(i) => GPRS + *i as usize
        }
    }
    pub fn from(idx: u32) -> Self {
        use Reg::*;
        match idx {
            0..=29 => R(idx as u8),
            30     => SP,
            31     => RZR,
            32     => PC,
            _      => ID(idx - GPRS as u32)
        }
    }
}
impl Ord for Reg {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index().cmp(&other.index())
    }
}
impl PartialOrd for Reg {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

#[derive(Copy, Clone)]
pub enum Const {
    Int(i64),
    Float(f64)
}

impl Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Const as C;
        match self {
            C::Int(v)   => write!(f, "{}", v),
            C::Float(v) => write!(f, "{}", v),
        }
    }
}