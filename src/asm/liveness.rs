// See https://www.cs.cornell.edu/courses/cs4120/2023sp/notes/#iterative
use super::cfg::CFG;
use super::asm::{AA, Reg};
use std::collections::{VecDeque, HashSet};
pub struct Liveness {
    pub lin:   Vec<Vec<Reg>>,
    pred:      Vec<Vec<usize>>,
    has:       Vec<HashSet<Reg>>,
    defs:      Vec<HashSet<Reg>>,
    used:      Vec<HashSet<Reg>>,
    queue:     VecDeque<(usize, Vec<Reg>)>
}
impl Liveness {
    pub fn new(cfg: &CFG) -> Self {
        let mut pred = vec![Vec::new(); cfg.nodes.len()];
        let lin = vec![Vec::new(); cfg.nodes.len()];
        let has = vec![HashSet::new(); cfg.nodes.len()];
        let mut defs: Vec<HashSet<Reg>> = vec![
            HashSet::new(); cfg.nodes.len()
        ];
        let mut used: Vec<HashSet<Reg>> = vec![
            HashSet::new(); cfg.nodes.len()
        ];
        let mut queue: VecDeque<(usize, Vec<Reg>)> = VecDeque::new();
        for i in 0..cfg.nodes.len() {
            let node = &cfg.nodes[i];
            let idx = node.idx;
            if let Some(t) = node.t {
                let tidx = cfg.nodes[t].idx;
                pred[tidx].push(idx);
            }
            if let Some(f) = node.f {
                let fidx = cfg.nodes[f].idx;
                pred[fidx].push(idx);
            }
            let asm = cfg.asm[idx];
            let (d, u) = Self::statement(asm);
            defs[idx] = HashSet::from_iter(d.into_iter());
            used[idx] = HashSet::from_iter(u.clone());
            queue.push_back((idx, u));
        }
        let mut ans = Self {
            lin,
            pred,
            has,
            defs,
            used,
            queue
        };
        ans.solve();
        return ans;
    }

    // When you understand what to update...
    pub fn update(&mut self) {}

    fn solve(&mut self) {
        while let Some((idx, delta)) = self.queue.pop_front() {
            let mut new_delta: Vec<Reg> = Vec::new();
            for change in delta {
                if !self.used[idx].contains(&change) &&
                    self.defs[idx].contains(&change) {
                    continue;
                }
                if self.has[idx].contains(&change) {
                    continue;
                }
                self.has[idx].insert(change);
                self.lin[idx].push(change);
                new_delta.push(change);
            }
            if new_delta.len() == 0 { continue }
            for p in &self.pred[idx] {
                self.queue.push_back((
                    *p,
                    new_delta.clone()
                ));
            }
        }
    }
    // Def, Use
    #[allow(unused_variables)]
    fn statement(asm: AA) -> (Vec<Reg>, Vec<Reg>) {
        use AA::*;
        use Reg::*;
        return match asm {
            Label(l)           => (vec![],   vec![]),
            Mov1(d, s)         => (vec![d],  vec![]),
            Mov2(d, s)         => (vec![d],  vec![s]),
            Add1(d, l, r)      => (vec![d],  vec![l]),
            Add2(d, l, r)      => (vec![d],  vec![l, r]),
            Sub1(d, l, r)      => (vec![d],  vec![l]),
            Sub2(d, l, r)      => (vec![d],  vec![l, r]),
            Neg1(d, s)         => (vec![d],  vec![]),
            Neg2(d, s)         => (vec![d],  vec![s]),
            SMAddL(d, l, m, r) => (vec![d],  vec![l, m, r]),
            SMNegL(d, l, r)    => (vec![d],  vec![l, r]),
            SMSubL(d, l, m, r) => (vec![d],  vec![l, r]),
            SMulL(d, l, r)     => (vec![d],  vec![l, r]),
            SDiv(d, l, r)      => (vec![d],  vec![l, r]),
            And1(d, l, r)      => (vec![d],  vec![l]),
            And2(d, l, r)      => (vec![d],  vec![l, r]),
            Or1(d, l, r)       => (vec![d],  vec![l]),
            Or2(d, l, r)       => (vec![d],  vec![l, r]),
            Mvn1(d, s)         => (vec![d],  vec![]),
            Mvn2(d, s)         => (vec![d],  vec![s]),
            B(l)               => (vec![],   vec![]),
            BL(l)              => (vec![],   vec![]),
            CBZ(l)             => (vec![],   vec![]),
            CBNZ(l)            => (vec![],   vec![]),
            CMP1(d, s)         => (vec![d],  vec![]),
            CMP2(d, s)         => (vec![d],  vec![s]),
            CSET(d, s)         => (vec![d],  vec![]),
            LDR1(d, l, r)      => (vec![d],  vec![l]),
            LDR2(d, s)         => (vec![d],  vec![s]),
            STR1(d, l, r)      => (vec![d],  vec![l]),
            STR2(d, s)         => (vec![d],  vec![s]),
            Ret                => (vec![SP], vec![R(29)])
        };
    }
}