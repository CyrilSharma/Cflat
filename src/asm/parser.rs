use std::fmt;
use super::asm::{AA, Reg, Const, GPRS};

pub enum ParseError {
    Quote(u32, String),
    Register(u32, String),
    Delimiter(u32, String),
    InvalidChar(u32, String),
    MissingOp(String),
    MissingArgs(String),
    InvalidOp(String)
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParseError as P;
        match &self {
            P::Quote(i, s) => {
                writeln!(f, "Missing closing quote.")?;
                writeln!(f, "{}", s)?;
                writeln!(f, "{}^", "-".repeat(*i as usize))?;
            },
            P::Register(i, s)    => {
                writeln!(f, "Invalid Register Used.")?;
                writeln!(f, "{}", s)?;
                writeln!(f, "{}^", "-".repeat(*i as usize))?;
            },
            P::Delimiter(i, s)   => {
                writeln!(f, "Missing comma / other delimiter used.")?;
                writeln!(f, "{}", s)?;
                writeln!(f, "{}^", "-".repeat(*i as usize))?;
            },
            P::InvalidChar(i, s) => {
                writeln!(f, "Invalid char encountered in asm")?;
                writeln!(f, "{}", s)?;
                writeln!(f, "{}^", "-".repeat(*i as usize))?;
            }
            P::MissingOp(s) => {
                writeln!(f, "Asm is missing op code!")?;
                writeln!(f, "{}", s)?;
                writeln!(f, "^")?;
            },
            P::MissingArgs(s) => {
                writeln!(f, "Asm is missing arguments!")?;
                writeln!(f, "{}", s)?;
                writeln!(f, "{}^", "-".repeat(s.len()))?;
            },
            P::InvalidOp(s) => {
                writeln!(f, "Asm does not use a supported op! Note that Control Flow and Labels are not currently supported.")?;
                writeln!(f, "{}", s)?;
                writeln!(f, "^")?;
            }
        }
        Ok(())
    }
}
impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

enum State {
    Quoted, 
    Unquoted
}
// Parses a single line.
pub fn parse(asm: String) -> Result<AA, ParseError> {
    use State as S;
    use ParseError as P;
    let mut state = S::Unquoted;
    let mut quoteidx = 0;
    let mut token = String::new();
    let mut tokens: Vec<String> = Vec::new();
    let mut tokenidxs = Vec::new();
    for (i, c) in asm.chars().enumerate() {
        match state {
            S::Unquoted => {
                match c {
                    'a'..='z' | 'A'..='Z' |
                    '0'..='9' | '#' => token.push(c),
                    '\"' => {
                        state = S::Quoted;
                        quoteidx = i;
                    },
                    ' ' | ','  => {
                        if token.len() == 0 { continue }
                        tokens.push(token);
                        tokenidxs.push(i - 1);
                        token = String::new();
                    },
                    _ => return Err(P::InvalidChar(
                        i as u32, asm
                    )),
                }
            },
            // We'll handle escaped stuff another day.
            S::Quoted => {
                match c {
                    '\"' => {
                        state = S::Unquoted;
                        tokens.push(token);
                        tokenidxs.push(i);
                        token = String::new();
                    },
                    _    => token.push(c)
                }
            },
        }
    }
    tokens.push(token);
    match state {
        S::Unquoted => (),
        S::Quoted => return Err(P::Quote(
            quoteidx as u32, asm
        ))
    }

    let op = match tokens.get(0) {
        None => return Err(P::MissingOp(asm)),
        Some(s) => s
    };

    let access = |idx: usize| {
        match &tokens.get(idx) {
            None => Err(P::MissingArgs(asm.clone())),
            Some(s) => Ok(s.clone())
        }
    };

    let reg = |idx| {
        use Reg as R;
        let token = access(idx + 1)?;
        match token.as_ref() {
            "SP"  => return Ok(R::SP),
            "RZR" => return Ok(R::RZR),
            "PC"  => return Ok(R::PC),
            _ => ()
        };
        match token.chars().nth(0) {
            Some(s) if s == 'R' => (),
            _ => return Err(P::Register(
                tokenidxs[idx] as u32,
                asm.clone()
            )),
        }
        match token[1..].parse::<u8>() {
            Ok(i) if i < GPRS as u8 => Ok(R::R(i)),
            _ => Err(P::Register(
                tokenidxs[idx] as u32,
                asm.clone()
            )),
        }
    };

    let con = |idx| {
        use Const as C;
        let token = access(idx + 1)?;
        match token.chars().nth(0) {
            Some(c) if c == '#' => (),
            _ => return Err(P::Register(
                tokenidxs[idx] as u32,
                asm.clone()
            )),
        }
        let no_prefix = token.trim_start_matches("#0x");
        if let Ok(i) = token[1..].parse::<i64>() {
            Ok(C::Int(i))
        } else if let Ok(h) = i64::from_str_radix(&no_prefix, 16) {
            Ok(C::Int(h))
        } else if let Ok(f) = token[1..].parse::<f64>() {
            Ok(C::Float(f))
        } else {
            Err(P::Register(
                tokenidxs[idx] as u32,
                asm.clone()
            ))
        }
    };

    use AA as A;
    // We don't support Labels right now.
    // This is because Labels have IDs, and we don't know what IDs are valid.
    // Could be fixed if we reserved special names for inline asm IDs.
    return Ok(match op.as_ref() {
        "mov"  if con(1).is_ok() => A::Mov1(reg(0)?, con(1)?),
        "mov"  if reg(1).is_ok() => A::Mov2(reg(0)?, reg(1)?),
        "add"  if con(2).is_ok() => A::Add1(reg(0)?, reg(1)?, con(2)?), 
        "add"  if reg(2).is_ok() => A::Add2(reg(0)?, reg(1)?, reg(2)?),
        "sub"  if con(2).is_ok() => A::Sub1(reg(0)?, reg(1)?, con(2)?), 
        "sub"  if reg(2).is_ok() => A::Sub2(reg(0)?, reg(1)?, reg(2)?),
        "neg"  if con(1).is_ok() => A::Neg1(reg(0)?, con(1)?),
        "neg"  if reg(1).is_ok() => A::Neg2(reg(0)?, reg(1)?),
        "smaddl" => A::SMAddL(reg(0)?, reg(1)?, reg(2)?, reg(3)?),
        "smsubl" => A::SMSubL(reg(0)?, reg(1)?, reg(2)?, reg(3)?),
        "smnegl" => A::SMNegL(reg(0)?, reg(1)?, reg(2)?),
        "smull"  => A::SMulL(reg(0)?, reg(1)?, reg(2)?),
        "sdiv"   => A::SDiv(reg(0)?, reg(1)?, reg(2)?),
        "and"  if con(1).is_ok() => A::And1(reg(0)?, reg(1)?, con(2)?),
        "and"  if reg(1).is_ok() => A::And2(reg(0)?, reg(1)?, reg(2)?),
        "or"   if con(1).is_ok() => A::And1(reg(0)?, reg(1)?, con(2)?),
        "or"   if reg(1).is_ok() => A::And2(reg(0)?, reg(1)?, reg(2)?),
        "movn" if con(1).is_ok() => A::Mvn1(reg(0)?, con(1)?),
        "movn" if reg(1).is_ok() => A::Mvn2(reg(0)?, reg(1)?),
        "cmp"  if con(1).is_ok() => A::CMP1(reg(0)?, con(1)?),
        "cmp"  if reg(1).is_ok() => A::CMP2(reg(0)?, reg(1)?),
        "ldr"  if access(2).is_ok()  => A::LDR1(reg(0)?, reg(1)?, con(2)?),
        "ldr"  if access(2).is_err() => A::LDR2(reg(0)?, reg(1)?),
        "str"  if access(2).is_ok()  => A::STR1(reg(0)?, reg(1)?, con(2)?),
        "str"  if access(2).is_err() => A::STR2(reg(0)?, reg(1)?),
        "svc"  => A::SVC(con(0)?),
        "ret"  => A::Ret,
        _ => return Err(P::InvalidOp(asm))
    });
}