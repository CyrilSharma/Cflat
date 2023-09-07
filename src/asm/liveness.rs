// See https://www.cs.cornell.edu/courses/cs4120/2023sp/notes/#iterative
use super::cfg::{CFG, Node};
use super::asm::{AA, Reg};
use std::collections::{VecDeque, HashSet};
pub struct Liveness;
impl Liveness {
    pub fn compute(cfg: CFG) -> Vec<(AA, Vec<bool>, Vec<bool>)> {
        let mut pred = vec![Vec::new();     cfg.nodes.len()];
        // It may not be necessary to decouple the two.
        let mut lin  = vec![Vec::new();     cfg.nodes.len()];
        let mut has  = vec![HashSet::new(); cfg.nodes.len()];
        // There's at most three uses per asm instruction, hashset unnecessary.
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
            let (_, uses) = cfg.asm[idx].defuse();
            queue.push_back((idx, uses));
        }
        while let Some((idx, delta)) = queue.pop_front() {
            let mut new_delta: Vec<Reg> = Vec::new();
            for change in delta {
                // If we use it, OR we don't overwrite it.
                let (defs, uses) = cfg.asm[idx].defuse();
                if !has[idx].contains(&change) &&
                    (uses.contains(&change) ||
                    !defs.contains(&change)) {
                    lin[idx].push(change);
                    has[idx].insert(change);
                    new_delta.push(change);
                }
            }
            if new_delta.len() == 0 { continue }
            for p in &pred[idx] {
                queue.push_back((
                    *p, std::mem::take(&mut new_delta)
                ));
            }
        }

        let mut res = Vec::new();
        // We need to iterate over asm in order.
        let mut nodes: Vec<Node> = vec![
            Node { idx: usize::MAX, t: None, f: None};
            cfg.nodes.len()
        ];
        for node in &cfg.nodes {
            nodes[node.idx] = node.clone();
        }
        for (idx, node) in nodes.into_iter().enumerate() {
            assert!(node.idx == idx);
            use AA::*;
            // Insert Basic Block Pseudo-Ops
            let bb = if idx == 0 { true } else {
                match (&cfg.asm[idx - 1], &cfg.asm[idx]) {
                    (B(_) | BL(_), _) => true,
                    (_, Label(_))     => true,
                    _                 => false
                }
            };
            if bb {
                res.push((
                    BB(lin[idx].clone()),
                    vec![false; lin[idx].len()],
                    vec![]
                ));
            }

            // Process normal ASM.
            let (defs, uses) = cfg.asm[idx].defuse();
            let mut defdead = vec![true; defs.len()];
            for (i, reg) in defs.iter().enumerate() {
                if let Some(t) = node.t {
                    let tidx = cfg.nodes[t].idx;
                    defdead[i] &= !lin[tidx].contains(&reg);
                }
                if let Some(f) = node.f {
                    let fidx = cfg.nodes[f].idx;
                    defdead[i] &= !lin[fidx].contains(&reg);
                }
            }
            let mut usedead = vec![true; uses.len()];
            for (i, reg) in uses.iter().enumerate() {
                if let Some(t) = node.t {
                    let tidx = cfg.nodes[t].idx;
                    usedead[i] &= !lin[tidx].contains(&reg);
                }
                if let Some(f) = node.f {
                    let fidx = cfg.nodes[f].idx;
                    usedead[i] &= !lin[fidx].contains(&reg);
                }
            }
            res.push((
                cfg.asm[idx].clone(),
                defdead,
                usedead
            ));
        }
        return res;
    }
}