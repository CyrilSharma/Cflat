// See https://www.cs.cornell.edu/courses/cs4120/2023sp/notes/#iterative
use super::cfg::CFG;
use super::asm::{AA, Reg};
use std::collections::{VecDeque, HashSet};
pub struct Liveness;
impl Liveness {
    pub fn compute(cfg: &CFG) -> Vec<Vec<Reg>> {
        let mut lin = vec![Vec::new(); cfg.nodes.len()];
        let mut has = vec![HashSet::new(); cfg.nodes.len()];
        let mut defs: Vec<HashSet<Reg>> = vec![
            HashSet::new(); cfg.nodes.len()
        ];
        let mut queue: VecDeque<(usize, Vec<Reg>)> = VecDeque::new();
        for i in 0..cfg.nodes.len() {
            queue.push_back((i, vec![]));
            let node = &cfg.nodes[i];
            let (d, u) = Self::statement(node.asm);
            lin[i].extend(u);
            defs[i] = HashSet::from_iter(
                d.iter().cloned()
            );
        }
        while let Some((idx, delta)) = queue.pop_front() {
            let mut new_delta: Vec<Reg> = Vec::new();
            for change in delta {
                if defs[idx].contains(&change) {
                    continue;
                }
                if has[idx].contains(&change) {
                    continue;
                }
                has[idx].insert(change);
                lin[idx].push(change);
                new_delta.push(change);
                println!("I HAPPENNED");
            }
            if new_delta.len() == 0 { continue }
            queue.push_back((idx, new_delta));
        }
        return lin;
    }
    // Def, Use
    #[allow(unused_variables)]
    fn statement(asm: AA) -> (Vec<Reg>, Vec<Reg>) {
        use AA::*;
        return match asm {
            Label(l)           => (vec![],  vec![]),
            Mov1(d, s)         => (vec![d], vec![]),
            Mov2(d, s)         => (vec![d], vec![s]),
            Add1(d, l, r)      => (vec![d], vec![]),
            Add2(d, l, r)      => (vec![d], vec![l, r]),
            Sub1(d, l, r)      => (vec![d], vec![l]),
            Sub2(d, l, r)      => (vec![d], vec![l, r]),
            Neg1(d, s)         => (vec![d], vec![]),
            Neg2(d, s)         => (vec![d], vec![s]),
            SMAddL(d, l, m, r) => (vec![d], vec![l, m, r]),
            SMNegL(d, l, r)    => (vec![d], vec![l, r]),
            SMSubL(d, l, m, r) => (vec![d], vec![l, r]),
            SMulL(d, l, r)     => (vec![d], vec![l, r]),
            SDiv(d, l, r)      => (vec![d], vec![l, r]),
            And1(d, l, r)      => (vec![d], vec![l]),
            And2(d, l, r)      => (vec![d], vec![l, r]),
            Or1(d, l, r)       => (vec![d], vec![l]),
            Or2(d, l, r)       => (vec![d], vec![l, r]),
            Mvn1(d, s)         => (vec![d], vec![]),
            Mvn2(d, s)         => (vec![d], vec![s]),
            B(l)               => (vec![],  vec![]),
            BL(l)              => (vec![],  vec![]),
            CBZ(l)             => (vec![],  vec![]),
            CBNZ(l)            => (vec![],  vec![]),
            CMP1(d, s)         => (vec![d], vec![]),
            CMP2(d, s)         => (vec![d], vec![s]),
            CSET(d, s)         => (vec![d], vec![]),
            LDR1(d, l, r)      => (vec![d], vec![l]),
            LDR2(d, s)         => (vec![d], vec![s]),
            STR1(d, l, r)      => (vec![d], vec![l]),
            STR2(d, s)         => (vec![d], vec![s]),
            Ret                => (vec![],  vec![]),
        };
    }
}