use std::collections::BTreeMap;
use crate::ir::ir::{self, Statement, Expr, Operator};
use super::asm::{self, AA, Reg, CC, Const};
use crate::registry::Registry;

type ID = u32;

// The better way is to not clone every time you return
// Allocate Vec<AA> in a linked list, prepend or post-pend nodes.
// The issue is this sounds like hell when it comes to rust...
#[derive(Clone)]
struct Info {
    cost: u32,
    temp: ID,
    asm : Vec<AA>
}
impl Info {
    pub fn new(reg: u32) -> Self {
        Info {
            cost: u32::MAX,
            temp: reg,
            asm:  Vec::new()
        }
    }
    pub fn update(&mut self, cost: u32, asm: Vec<AA>) {
        if cost < self.cost {
            self.cost = cost;
            self.asm = asm;
        }
    }
}
pub struct Translator { 
    opt:    BTreeMap<usize, Info>,
    frames: Vec<usize>,
    count:  usize,
    retid:  u32,
    main:   bool
}

impl Translator {
    pub fn new(reg: &Registry, flist: Vec<usize>) -> Self { 
        Self { 
            opt:    BTreeMap::new(),
            frames: flist,
            count:  reg.nids as usize,
            retid:  reg.ret,
            main:   false
        }
    }
    pub fn translate(r: &mut Registry, flist: Vec<usize>, 
        stmts: Vec<Box<Statement>>) -> Vec<AA> {
        let mut t = Self { 
            opt:    BTreeMap::new(),
            frames: flist,
            count:  r.nids as usize,
            retid:  r.ret,
            main:   false
        };
        let mut res = Vec::<AA>::new();
        for s in stmts {
            res.extend(t.statement(&s));
        }
        r.nids = t.count as u32;
        return res;
    }
    fn statement(&mut self, s: &Statement) -> Vec<AA> {
        use Statement::*;
        match s {
            Expr(e)        => self.call(e),
            Move(d, s)     => self._move(d, s),
            CJump(c, t, _) => self.cjump(c, *t),
            Return(r)      => self._return(r),
            Function(f, v) => self.function(*f, v),
            Jump(j)        => vec![AA::B1(*j)],
            Label(l)       => vec![AA::Label(*l)],
            Asm(a)         => vec![a.clone()],
            _ => unreachable!()
        }
    }
    fn function(&mut self, f: u32, v: &Vec<u32>) -> Vec<AA> {
        // Set up frame.
        use asm::Const as C;
        self.main = f == 0;
        let mut asm = vec![AA::Label(f)];
        if !self.main {
            asm.push(AA::STR1(Reg::R(29), Reg::SP, C::Int(-16)));
            asm.push(AA::Sub1(Reg::SP, Reg::SP, C::Int(-16)));
        }
        for (i, t) in v.iter().enumerate() {
            if i >= 8 { panic!("Unimplemented!") }
            asm.push(AA::Mov2(
                Reg::ID(*t),
                Reg::R(i as u8)
            ));
        }
        return asm;
    }
    fn _return(&mut self, r: &Option<Box<Expr>>) -> Vec<AA> {
        match r { 
            None => return vec![AA::Ret],
            Some(e) if !self.main => {
                let Info { mut asm, temp, .. } = self.expression(e);
                asm.push(AA::Mov2(Reg::R(0), Reg::ID(temp)));
                asm.push(AA::Ret);
                return asm;
            },
            Some(e) => {
                let Info { mut asm, temp, .. } = self.expression(e);
                asm.push(AA::Mov2(Reg::R(0), Reg::ID(temp)));
                asm.push(AA::Mov1(Reg::R(16), Const::Int(1)));
                asm.push(AA::SVC(Const::Int(128)));
                return asm;
            }
        }
    }
    fn call(&mut self, e: &Expr) -> Vec<AA> {
        use ir::Expr::*;
        use Reg::*;
        let Call(f, args) = e else { unreachable!(); };
        let mut asm = Vec::<AA>::new();
        // You can actually avoid pre-coloring the registers.
        // Make each call load into one set of temps, 
        // Pull from the same set of temps here
        let arg_reg = vec![R(0), R(1), R(2), R(3), R(4), R(5), R(6), R(7)];
        if args.len() > 8 { panic!("Too many arguments!"); }
        for i in 0..args.len() {
            let Temp(r) = *args[i] else { unreachable!() };
            asm.push(AA::Mov2(arg_reg[i as usize], Reg::ID(r)));
        }
        asm.push(AA::BL(*f));
        return asm;
    }
    fn _move(&mut self, d: &Expr, s: &Expr) -> Vec<AA> {
        use Expr::*;
        match (d, s) {
            (Mem(t), e)   => {
                let Info { temp: mtemp, asm: masm, .. } = self.expression(t);
                let Info { temp: etemp, asm: easm, .. } = self.expression(e);
                let mut asm = masm;
                asm.extend(easm);
                asm.push(AA::STR2(
                    Reg::ID(mtemp),
                    Reg::ID(etemp)
                ));
                return asm;
            },
            (Temp(a), e)    => {
                let Info { cost: _, temp, mut asm } = self.expression(e);
                let r = if *a == self.retid { Reg::R(0) } else { Reg::ID(*a) };
                asm.push(AA::Mov2(r, Reg::ID(temp)));
                return asm;
            },
            _ => unreachable!()
        }
    }
    fn cjump(&mut self, j: &Expr, t: ir::Label) -> Vec<AA> {
        let Info { mut asm, temp, .. } = self.expression(j);
        asm.push(AA::CBZ(Reg::ID(temp), t));
        return asm;
    }
    fn expression(&mut self, e: &Expr) -> Info {
        let nid = e.addr();
        match self.opt.get(&nid) {
            None => (),
            Some(s) => return s.clone()
        }
        use Expr::*;
        let ans = match e {
            UnOp(op, e)        => self.unary(*op, e,),
            BinOp(l, op, r)    => self.binary(l, *op, r),
            Mem(m)             => self.mem(m),
            Address(e)         => self.address(e),
            Const(c)           => self._const(c),
            Temp(i)            => self._temp(*i),
            _ => unreachable!()
        };
        self.opt.insert(nid, ans.clone());
        return ans;
    }
    fn _const(&mut self, p: &ir::Primitive) -> Info {
        use ir::Primitive as P;
        use asm::Const as C;
        let c = match p {
            P::Int(i)   => C::Int(*i as i64),
            P::Float(f) => C::Float(*f as f64),
        };
        let res = self.create_temp();
        let mut ans = Info::new(res);
        let asm = vec![AA::Mov1(Reg::ID(res), c)];
        ans.update(asm.len() as u32, asm);
        return ans;
    }
    fn _temp(&mut self, i: u32) -> Info {
        let res = self.create_temp();
        let mut ans = Info::new(res);
        let r = if i == self.retid { Reg::R(0) } else { Reg::ID(i) };
        let asm = vec![AA::Mov2(Reg::ID(res), r)];
        // If the variable exists it must be stored on the stack.
        /* if self.frames[i as usize] != usize::MAX {
            asm.push(AA::STR1(r, Reg::SP, self.frames[i as usize]));
        } */
        ans.update(asm.len() as u32, asm);
        return ans;
    }
    fn unary(&mut self, op: Operator, e: &Expr) -> Info {
        let res = self.create_temp();
        let mut ans = Info::new(res);
        use Expr::*;
        case!({ // LOAD TEMP <== Neg-Mul
            if op != Operator::Neg { break };
            let BinOp(l, Operator::Mul, r) = e else { break };
            let Info { temp: ltmp, asm: lasm, .. } = self.expression(l);
            let Info { temp: rtmp, asm: rasm, .. } = self.expression(r);
            let mut asm = lasm.clone();
            asm.extend(rasm);
            asm.push(AA::SMNegL(
                Reg::ID(res),
                Reg::ID(ltmp),
                Reg::ID(rtmp)
            ));
            ans.update(asm.len() as u32, asm);
        });
        case!({ // LOAD TEMP <== Neg
            if op != Operator::Neg { break };
            let Info { cost: _, temp, asm } = self.expression(e);
            let mut asm = asm.clone();
            asm.push(AA::Neg2(
                Reg::ID(res),
                Reg::ID(temp)
            ));
            ans.update(asm.len() as u32, asm);
        });
        case!({ // MOV TEMP <== ~CONST
            use ir::Primitive as P;
            use asm::Const as C;
            if op != Operator::Not { break };
            let Const(p) = e else { break };
            let asm = vec![AA::Mvn1(
                Reg::ID(res),
                match p {
                    P::Int(i)   => C::Int(*i as i64),
                    P::Float(f) => C::Float(*f as f64),
                }
            )];
            ans.update(asm.len() as u32, asm);
        });
        case!({ // MOV TEMP <== ~EXPR
            if op != Operator::Not { break };
            let Info { cost: _, temp, asm } = self.expression(e);
            let mut asm = asm.clone();
            asm.push(AA::Mvn2(
                Reg::ID(res),
                Reg::ID(temp)
            ));
            ans.update(asm.len() as u32, asm);
        });
        return ans;
    }
    fn binary(&mut self, l: &Expr, op: Operator, r: &Expr) -> Info {
        let res = self.create_temp();
        let mut ans = Info::new(res);
        use Expr::*;
        case!({ // LOAD TEMP <== MUL-ADD
            if op != Operator::Add && op != Operator::Sub { break };
            let BinOp(l2, Operator::Mul, r2) = r else { break };
            let Info { temp: ltmp,  asm: lasm,  .. } = self.expression(l);
            let Info { temp: l2tmp, asm: l2asm, .. } = self.expression(&l2);
            let Info { temp: r2tmp, asm: r2asm, .. } = self.expression(&r2);
            let mut asm = lasm.clone();
            asm.extend(l2asm);
            asm.extend(r2asm);
            if op == Operator::Add {
                asm.push(AA::SMAddL(
                    Reg::ID(res),   Reg::ID(l2tmp),
                    Reg::ID(r2tmp), Reg::ID(ltmp)
                ));
            } else {
                asm.push(AA::SMSubL(
                    Reg::ID(res),   Reg::ID(l2tmp),
                    Reg::ID(r2tmp), Reg::ID(ltmp)
                ));
            }
            ans.update(asm.len() as ID, asm);
        });
        case!({ // LOAD TEMP <== Expr OP Expr
            let Info { temp: ltmp, asm: lasm, .. } = self.expression(l);
            let Info { temp: rtmp, asm: rasm, .. } = self.expression(r);
            let mut asm = lasm.clone();
            asm.extend(rasm.clone());
            asm.extend(match op {
                Operator::Add => vec![AA::Add2(
                    Reg::ID(res),
                    Reg::ID(ltmp),
                    Reg::ID(rtmp),
                )],
                Operator::Sub => vec![AA::Sub2(
                    Reg::ID(res),
                    Reg::ID(ltmp),
                    Reg::ID(rtmp),
                )],
                Operator::Mul => vec![AA::SMulL(
                    Reg::ID(res),
                    Reg::ID(ltmp),
                    Reg::ID(rtmp),
                )],
                Operator::Div => vec![AA::SDiv(
                    Reg::ID(res),
                    Reg::ID(ltmp),
                    Reg::ID(rtmp),
                )],
                Operator::And => vec![AA::And2(
                    Reg::ID(res),
                    Reg::ID(ltmp),
                    Reg::ID(rtmp),
                )],
                Operator::Or => vec![AA::Or2(
                    Reg::ID(res),
                    Reg::ID(ltmp),
                    Reg::ID(rtmp),
                )],
                Operator::Eq => vec![
                    AA::CMP2(
                        Reg::ID(ltmp),
                        Reg::ID(rtmp),
                    ),
                    AA::CSET(
                        Reg::ID(res),
                        CC::EQ
                    )
                ],
                Operator::Neq => vec![
                    AA::CMP2(
                        Reg::ID(ltmp),
                        Reg::ID(rtmp),
                    ),
                    AA::CSET(
                        Reg::ID(res),
                        CC::NE
                    )
                ],
                Operator::Leq => vec![
                    AA::CMP2(
                        Reg::ID(ltmp),
                        Reg::ID(rtmp),
                    ),
                    AA::CSET(
                        Reg::ID(res),
                        CC::LE
                    )
                ],
                Operator::Geq => vec![
                    AA::CMP2(
                        Reg::ID(ltmp),
                        Reg::ID(rtmp),
                    ),
                    AA::CSET(
                        Reg::ID(res),
                        CC::GE
                    )
                ],
                Operator::Lt => vec![
                    AA::CMP2(
                        Reg::ID(ltmp),
                        Reg::ID(rtmp),
                    ),
                    AA::CSET(
                        Reg::ID(res),
                        CC::LT
                    )
                ],
                Operator::Gt => vec![
                    AA::CMP2(
                        Reg::ID(ltmp),
                        Reg::ID(rtmp),
                    ),
                    AA::CSET(
                        Reg::ID(res),
                        CC::GT
                    )
                ],
                Operator::Mod => {
                    let t1 = self.create_temp();
                    vec![
                        AA::SDiv(
                            Reg::ID(t1),
                            Reg::ID(ltmp),
                            Reg::ID(rtmp),
                        ),
                        AA::SMulL(
                            Reg::ID(res),
                            Reg::ID(t1),
                            Reg::ID(rtmp)
                        ),
                        AA::Sub2(
                            Reg::ID(res),
                            Reg::ID(ltmp),
                            Reg::ID(res)
                        )
                    ]
                },
                _ => unreachable!()
            });
            ans.update(asm.len() as ID, asm);
        });
        return ans;
    }
    fn mem(&mut self, m: &Expr) -> Info {
        let res = self.create_temp();
        let mut ans = Info::new(res);
        // use Expr::*;
        case!({ // LOAD TEMP <== Neg-Mul
            /* let Const(p) = m else { break };
            let asm = vec![AA::LDR1(
                Reg::ID(res),
                p.bits()
            )];
            update(asm.len() as u32, asm); */
        });
        case!({ // LOAD TEMP <== Neg-Mul
            let Info { cost: _, temp, asm } = self.expression(m);
            let mut asm = asm.clone();
            asm.push(AA::LDR2(
                Reg::ID(res),
                Reg::ID(temp)
            ));
            ans.update(asm.len() as u32, asm);
        });
        return ans;
    }
    fn address(&mut self, e: &Expr) -> Info {
        use Expr::*;
        use asm::Const as C;
        let res = self.create_temp();
        let mut ans = Info::new(res);
        match e {
            Temp(i) => {
                let asm = vec![AA::LDR1(
                    asm::Reg::ID(res), Reg::R(29),
                    C::Int(self.frames[*i as usize] as i64)
                )];
                ans.update(asm.len() as u32, asm);
            }
            Mem(e) => return self.expression(e),
            _ => unreachable!()
        }
        return ans;
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