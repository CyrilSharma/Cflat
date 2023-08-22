use std::collections::{BTreeMap};
use crate::ir::{self, Statement, Expr, Operator};
use crate::aa::{self, AA, Reg};

type ID = u32;
type Info = (u32, ID, Vec<AA>);
pub struct Translator { 
    opt:   BTreeMap<usize, Info>,
    count: usize
}

impl Translator {
    pub fn new(cnt: usize) -> Self { 
        Self { 
            opt:   BTreeMap::new(),
            count: cnt
        }
    }
    pub fn translate(&mut self, stmts: &[Statement]) -> Vec<AA> {
        let mut res = Vec::<AA>::new();
        for s in stmts {
            res.extend(self.statement(s));
        }
        return res;
    }
    fn statement(&mut self, s: &Statement) -> Vec<AA> {
        use Statement::*;
        match s {
            Expr(e)        => self.call(e),
            Move(d, s)     => self._move(d, s),
            CJump(c, t, _) => self.cjump(c, *t),
            Jump(j)   => vec![AA::B(*j)],
            Label(l)  => vec![AA::Label(*l)],
            Return(r) => vec![AA::Ret],
            _ => unreachable!()
        }
    }
    fn call(&mut self, e: &Expr) -> Vec<AA> {
        use ir::Expr::*;
        use aa::Reg::*;
        let Call(f, args) = e else { unreachable!(); };
        let mut asm = Vec::<AA>::new();
        let arg_reg = vec![R0, R1, R2, R3, R4, R5, R6, R7];
        if args.len() > 8 { panic!("Too many arguments!"); }
        for i in 0..args.len() {
            let Temp(i) = args[i] else { unreachable!() };
            asm.push(AA::Mov2(arg_reg[i as usize], Reg::ID(i)));
        }
        asm.push(AA::BL(*f));
        return asm;
    }
    fn _move(&mut self, d: &Expr, s: &Expr) -> Vec<AA> {
        let mut bc: u32 = u32::MAX;
        let mut basm: Vec<AA> = Vec::new();
        let update = |c: u32, asm: Vec<AA>| {
            if c >= bc { return }
            (bc, basm) = (c, asm);
        };
        use Expr::*;
        match (d, s) {
            (Temp(a), e)    => {
                let (_, temp, easm) = self.expression(e);
                let asm = easm.clone();
                asm.push(AA::Mov2(Reg::ID(*a), Reg::ID(*temp)));
                update(asm.len() as u32, asm);
                return asm;
            },
            (Mem(T), e)   => {
                let (_, mtemp, masm) = self.expression(T);
                let (_, etemp, easm) = self.expression(e);
                let mut asm = masm.clone();
                asm.extend(*easm);
                asm.push(AA::Mov2(Reg::ID(*mtemp), Reg::ID(*etemp)));
                update(asm.len() as u32, asm);
                return asm;
            }
            _ => unreachable!()
        }
    }
    fn cjump(&mut self, j: &Expr, t: ir::Label) -> Vec<AA> {
        let (_, etemp, easm) = self.expression(j);
        let mut asm = easm.clone();
        asm.push(AA::CBNZ(t));
        return asm;
    }
    fn expression(&mut self, e: &Expr) -> &Info {
        let nid = e.addr();
        match self.opt.get(&nid) {
            None => (),
            Some(s) => return s
        }
        use Expr::*;
        match e {
            UnOp(op, e)        => self.unary(*op, e, nid),
            BinOp(l, op, r)    => self.binary(l, *op, r, nid),
            Mem(m)             => self.mem(m, nid),
            Address(e)         => self.address(e, nid),
            // Must be handled explicitly by calling methods.
            Const(_) | Temp(_) => unreachable!(),
            _ => unreachable!()
        }
    }
    fn unary(&mut self, op: Operator, e: &Expr, nid: usize) -> &Info {
        match self.opt.get(&nid) {
            None => (),
            Some(s) => return s
        }
        let mut bc: u32 = u32::MAX;
        let mut basm: Vec<AA> = Vec::new();
        let update = |c: u32, asm: Vec<AA>| {
            if c >= bc { return }
            (bc, basm) = (c, asm);
        };
        let res = self.create_temp();
        use Expr::*;
        case!({ // LOAD TEMP <== Neg-Mul
            if op != Operator::Neg { break };
            let BinOp(l, Operator::Mul, r) = e else { break };
            let (_, ltemp, lasm) = self.expression(&l);
            let (_, rtemp, rasm) = self.expression(&r);
            let asm = lasm.clone();
            asm.extend(*rasm);
            asm.push(AA::SMNegL(
                Reg::ID(res),
                Reg::ID(*ltemp), Reg::ID(*rtemp)
            ));
            update(asm.len() as u32, asm);
        });
        case!({ // LOAD TEMP <== Neg
            if op != Operator::Neg { break };
            let (_, temp, mut asm) = self.expression(e);
            asm.push(AA::Neg(
                Reg::ID(res),
                Reg::ID(*temp)
            ));
            update(asm.len() as u32, asm);
        });
        case!({ // MOV TEMP <== ~CONST
            if op != Operator::Not { break };
            let Const(p) = e else { break };
            asm.push(AA::Mvn2(
                Reg::ID(res),
                Reg::Const(p.bits())
            ));
            update(asm.len() as u32, asm);
        });
        case!({ // MOV TEMP <== ~EXPR
            if op != Operator::Not { break };
            let (_, temp, mut asm) = self.expression(e);
            asm.push(AA::Mvn2(
                Reg::ID(res),
                Reg::ID(*temp)
            ));
            update(asm.len() as u32, asm);
        });
        self.opt.insert(nid, (bc, res, basm));
        return self.opt.get(&nid).unwrap();
    }
    fn binary(&mut self, l: &Expr, op: Operator, r: &Expr, nid: usize) -> &Info {
        match self.opt.get(&nid) {
            None => (),
            Some(s) => return s
        }
        let mut bc: u32 = u32::MAX;
        let mut basm: Vec<AA> = Vec::new();
        let update = |c: u32, asm: Vec<AA>| {
            if c >= bc { return }
            (bc, basm) = (c, asm);
        };
        let res = self.create_temp();
        use Expr::*;
        case!({ // LOAD TEMP <== MUL-ADD
            if op != Operator::Add && op != Operator::Sub { break };
            let BinOp(l2, Operator::Mul, r2) = r else { break };
            let (_, ltmp, lasm)    = self.expression(l);
            let (_, l2temp, l2asm) = self.expression(&l2);
            let (_, r2temp, r2asm) = self.expression(&r2);
            let mut asm = lasm.clone();
            asm.extend(*l2asm);
            asm.extend(*r2asm);
            if op == Operator::Add {
                asm.push(AA::SMAddL(
                    Reg::ID(res),     Reg::ID(*ltmp),
                    Reg::ID(*l2temp), Reg::ID(*r2temp)
                ));
            } else {
                asm.push(AA::SMSubL(
                    Reg::ID(res),     Reg::ID(*ltmp),
                    Reg::ID(*l2temp), Reg::ID(*r2temp)
                ));
            }
            update(asm.len() as ID, asm);
        });
        case!({ // LOAD TEMP <== Expr OP Expr
            let (_, ltmp, lasm) = self.expression(l);
            let (_, rtmp, rasm) = self.expression(r);
            let mut asm = lasm.clone();
            asm.extend(rasm.clone());
            asm.extend(match op {
                Operator::Add => vec![AA::Add2(
                    Reg::ID(res),
                    Reg::ID(*ltmp),
                    Reg::ID(*rtemp),
                )],
                Operator::Sub => vec![AA::Sub2(
                    Reg::ID(res),
                    Reg::ID(*ltmp),
                    Reg::ID(*rtemp),
                )],
                Operator::Mul => vec![AA::SMulL(
                    Reg::ID(res),
                    Reg::ID(*ltmp),
                    Reg::ID(*rtemp),
                )],
                Operator::Div => vec![AA::SDiv(
                    Reg::ID(res),
                    Reg::ID(*ltmp),
                    Reg::ID(*rtemp),
                )],
                Operator::And => vec![AA::And2(
                    Reg::ID(res),
                    Reg::ID(*ltmp),
                    Reg::ID(*rtemp),
                )],
                Operator::Or => vec![AA::Or2(
                    Reg::ID(res),
                    Reg::ID(*ltmp),
                    Reg::ID(*rtemp),
                )],
                Operator::Eq => vec![
                    AA::CMP2(
                        Reg::ID(*ltmp),
                        Reg::ID(*rtemp),
                    ),
                    AA::CSET(
                        Reg::ID(res),
                        CC::EQ
                    )
                ],
                Operator::Neq => vec![
                    AA::CMP2(
                        Reg::ID(*ltmp),
                        Reg::ID(*rtemp),
                    ),
                    AA::CSET(
                        Reg::ID(res),
                        CC::NE
                    )
                ],
                Operator::Leq => vec![
                    AA::CMP2(
                        Reg::ID(*ltmp),
                        Reg::ID(*rtemp),
                    ),
                    AA::CSET(
                        Reg::ID(res),
                        CC::LE
                    )
                ],
                Operator::Geq => vec![
                    AA::CMP2(
                        Reg::ID(*ltmp),
                        Reg::ID(*rtemp),
                    ),
                    AA::CSET(
                        Reg::ID(res),
                        CC::GE
                    )
                ],
                Operator::Lt => vec![
                    AA::CMP2(
                        Reg::ID(*ltmp),
                        Reg::ID(*rtemp),
                    ),
                    AA::CSET(
                        Reg::ID(res),
                        CC::LT
                    )
                ],
                Operator::Gt => vec![
                    AA::CMP2(
                        Reg::ID(*ltmp),
                        Reg::ID(*rtemp),
                    ),
                    AA::CSET(
                        Reg::ID(res),
                        CC::GT
                    )
                ],
            });
            update(asm.len() as ID, asm);
        });
        self.opt.insert(nid, (bc, res, basm));
        return self.opt.get(&nid).unwrap();
    }
    fn mem(&mut self, m: &Expr, nid: usize) -> &Info {
        match self.opt.get(&nid) {
            None => (),
            Some(s) => return s
        }
        let mut bc: u32 = u32::MAX;
        let mut basm: Vec<AA> = Vec::new();
        let update = |c: u32, asm: Vec<AA>| {
            if c >= bc { return }
            (bc, basm) = (c, asm);
        };
        let res = self.create_temp();
        use Expr::*;
        case!({ // LOAD TEMP <== Neg-Mul
            let Const(p) = m else { break };
            let asm = asm.clone();
            asm.push(AA::LDR1(
                Reg::ID(res),
                Reg::Const(p.bits())
            ));
            update(asm.len() as u32, asm);
        });
        case!({ // LOAD TEMP <== Neg-Mul
            let (_, temp, asm) = self.expression(m);
            let asm = asm.clone();
            asm.push(AA::LDR2(
                Reg::ID(res),
                Reg::ID(temp)
            ));
            update(asm.len() as u32, asm);
        });
        self.opt.insert(nid, (bc, res, basm));
        return self.opt.get(&nid).unwrap();
    }
    fn address(&mut self, e: &Expr, nid: usize) -> &Info {
        let idx = self.count;
        self.expression(e);
        return todo!();
    }
    fn create_temp(&mut self) -> ID {
        self.count += 1;
        return (self.count - 1) as ID;
    }
}

macro_rules! case {
    ($code:block) => {
        loop {
            $code
            break;
        }
    }
}
pub(crate) use case;