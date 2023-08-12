use std::collections::{BTreeMap};
use crate::ir::{self, Statement, Expr, Operator};
use crate::aa::{AA, Reg};

type ID = u32;
type Info = (u32, ID, Vec<AA>);
pub struct Translator { 
    opt:   BTreeMap<usize, Info>,
    count: usize
}

impl Translator {
    pub fn new(cnt: usize) -> Self { 
        Self { 
            opt: BTreeMap::new(),
            count: cnt
        }
    }
    pub fn translate(&mut self, stmts: &Vec<Statement>) -> Vec<AA> {
        let mut res = Vec::<AA>::new();
        for s in stmts {
            res.extend(self.statement(s));
        }
        return res;
    }
    fn statement(&mut self, s: &Statement) -> Vec<AA> {
        use Statement::*;
        match s {
            Expr(e)        => {
                // Weird but lowered IR should only have Calls
                let ir::Expr::Call(f, args) = **e else { unreachable!(); };
                self.call(f, &args)
            },
            Move(d, s)     => self._move(d, s),
            Jump(j)        => self.jump(j),
            CJump(c, t, _) => self.cjump(c, *t),
            Label(l)  => vec![AA::Label(*l)],
            Return(r) => vec![AA::Ret],
            _ => unreachable!()
        }
    }
    fn call(&mut self, e: &Expr) -> &Info {
        let ir::Expr::Call(f, args) = e else { unreachable!(); };
        for e in args { self.expression(e); }
        return todo!();
    }
    fn _move(&mut self, d: &Expr, s: &Expr) {
        let mut bc: u32 = u32::MAX;
        let mut basm: Vec<AA> = Vec::new();
        let update = |c: u32, asm: Vec<AA>| {
            if c >= bc { return }
            (bc, basm) = (c, asm);
        };
        // Writing update(asm.len() as u32, asm) every time seems redundant,
        // but in the future the cost function may be more complex.
        use Expr::*;
        match (d, s) {
            (Temp(a), Const(b))  => {
                let asm = vec![AA::Mov1(Reg::ID(*a), b.bits())];
                update(asm.len() as u32, asm);
            },
            (Temp(a), Temp(b))   => {
                let asm = vec![AA::Mov2(Reg::ID(*a), Reg::ID(*b))];
                update(asm.len() as u32, asm);
            },
            (Temp(a), Mem(T))    => {
                if let Temp(i) = **T {
                    let asm = vec![AA::LDR2(Reg::ID(*a), Reg::ID(i))];
                    update(asm.len() as u32, asm);
                } else { 
                    let (_, temp, tasm) = self.expression(T);
                    let mut asm = tasm.clone();
                    asm.push(AA::Mov2(Reg::ID(*a), Reg::ID(*temp)));
                    update(asm.len() as u32, asm);
                }
            },
            (Temp(a), e)    => {
                let (_, temp, easm) = self.expression(e);
                let asm = easm.clone();
                asm.push(AA::Mov2(Reg::ID(*a), Reg::ID(*temp)));
                update(asm.len() as u32, asm);
            }
            (Mem(T), Const(b))   => {
                if let Temp(i) = **T {
                    let asm = vec![AA::STR2(Reg::ID(i), b.bits())];
                    update(asm.len() as u32, asm);
                } else { 
                    let (_, temp, tasm) = self.expression(T);
                    let mut asm = tasm.clone();
                    asm.push(AA::Mov2(Reg::ID(*temp), b.bits()));
                    update(asm.len() as u32, asm);
                };
            }
            (Mem(T), Temp(b))    => {
                let Temp(i) = **T else { unreachable!() };
                vec![AA::STR2(Reg::ID(i), Reg::ID(*b))]
            },
            (Mem(D), Mem(S)) => {
                let id = self.create_temp();
                let Temp(r1) = **D else { unreachable!() };
                let Temp(r2) = **S else { unreachable!() };
                vec![
                    AA::LDR2(Reg::ID(id), Reg::ID(r2)),
                    AA::STR2(Reg::ID(r1), Reg::ID(id)),
                ]
            }
        }
        //---------LHS = TEMP------------
        case!({ // STORE TEMP <== CONST
            let (Temp(a), Const(b)) = (d, s) else { break };
            let asm = vec![AA::Mov1(Reg::ID(*a), b.bits())];
            update(asm.len() as u32, asm);
        });
        case!({ // STORE TEMP <== TEMP
            let (Temp(a), Temp(b)) = (d, s) else { break };
            let asm = vec![AA::Mov2(Reg::ID(*a), Reg::ID(*b))];
            update(asm.len() as u32, asm);
        });
        case!({ // LOAD TEMP <== [ REG ]
            let (Temp(a), Mem(T)) = (d, s) else { break };
            let Temp(i) = **T else { break };
            let asm = vec![AA::LDR2(Reg::ID(*a), Reg::ID(i))];
            update(asm.len() as u32, asm);
        });
        case!({ // LOAD TEMP <== [ REG ]
            let (Temp(a), Mem(T)) = (d, s) else { break };
            let Temp(i) = **T else { break };
            let asm = vec![AA::LDR2(Reg::ID(*a), Reg::ID(i))];
            update(asm.len() as u32, asm);
        });
        case!({ // LOAD TEMP <== EXPR -- DEFAULT
            let Temp(a) = d else { break };
            let (t, mut asm) = self.expression(s);
            asm.push(AA::Mov2(Reg::ID(*a), Reg::ID(t)));
            update(asm.len() as u32, asm);
        });

        //---------LHS = MEM------------
        case!({ // STORE [ REG ] <== CONST
            let (Mem(T), Const(b)) = (d, s);
            let Temp(reg) = **T else { break };
            let t = self.temp();
            let asm = vec![
                AA::Mov1(Reg::ID(t), b.bits()),
                AA::STR2(Reg::ID(reg), Reg::ID(t))
            ];
            update(asm.len() as u32, asm);
        });
        case!({ // STORE [ REG ] <== TEMP
            let (Mem(T), Temp(b)) = (d, s) else { break };
            let Temp(i) = **T else { break };
            let asm = vec![AA::STR2(Reg::ID(i), Reg::ID(*b))];
            update(asm.len() as u32, asm);
        });
        case!({ // STORE [ REG ] <== EXPR
            let Mem(T) = d else { break };
            let Temp(i) = **T else { break };
            let (t, mut asm) = self.expression(s);
            asm.push(AA::STR2(Reg::ID(i), Reg::ID(t)));
            update(asm.len() as u32, asm);
        });
    }
    fn cjump(&mut self, j: &Expr, t: ir::Label, f: ir::Label) {
        let idx = self.count;
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
        case!({ // MOVN TEMP <== !CONST
            if op != Operator::Neg { break };
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
        // TODO ADD SUB OR etc.
        self.opt.insert(nid, (bc, res, basm));
        return self.opt.get(&nid).unwrap();
    }
    fn mem(&mut self, m: &Expr, nid: usize) -> &Info {
        let idx = self.count;
        self.expression(m);
        return todo!();
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