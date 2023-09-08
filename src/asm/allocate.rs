// See https://web.eecs.umich.edu/~mahlke/courses/583f12/reading/chaitin82.pdf
use crate::registry::Registry;

use super::asm::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::mem::swap;


type AdjMatrix = Vec<HashSet<usize>>;
type AdjList   = Vec<Vec<usize>>;

struct DSU { parent: Vec<i32> }
impl DSU {
    pub fn new(n: i32) -> Self {
        let parent = vec![-1; n as usize];
        return DSU { parent }
    }
    pub fn find(&mut self, idx: i32) -> i32 {
        if self.parent[idx as usize] < 0 { return idx }
        self.parent[idx as usize] = self.find(
            self.parent[idx as usize]
        );
        return self.parent[idx as usize];
    }
    pub fn merge(&mut self, l: i32, r: i32) {
        let mut large = self.find(l) as usize;
        let mut small = self.find(r) as usize;
        if large == small { return }
        if self.parent[small] < self.parent[large] {
            swap(&mut small, &mut large);
        }
        self.parent[large] += self.parent[small];
        self.parent[small] = large as i32;
    }
    // Forces r to become the child of f.
    pub fn mergef(&mut self, l: i32, r: i32) {
        let large = self.find(l) as usize;
        let small = self.find(r) as usize;
        self.parent[large] += self.parent[small];
        self.parent[small] = large as i32;
    }
}
struct Node {
    deg:   u32,
    pos:   u32,
}

pub fn allocate(
        r: &mut Registry,
        mut live: Vec<(AA, Vec<bool>, Vec<bool>)>,
    ) -> Vec<AA> {
    let (mut amat, mut alist) = build_graph(r.nids, &mut live);
    coalesce_graph(&mut live, &mut alist, &mut amat);
    let colors = color_graph(alist);
    rewrite(&mut live, |r: Reg| {
        return Reg::from(colors[r.index()] as u32);
    });
    return live.into_iter().map(|x| x.0).collect();
}

pub fn coalesce_graph(
        live:  &mut Vec<(AA, Vec<bool>, Vec<bool>)>,
        alist: &mut AdjList,
        amat:  &mut AdjMatrix
    ) {
    // Things may get mapped multiple times.
    // Use a dsu to keep track.
    let mut dsu = DSU::new(alist.len() as i32);
    // You can also iterate until you reach a fixed point.
    for _ in 0..=1 {
        let mut f   = HashMap::new();
        for (ins, _, _) in live.iter() {
            let AA::Mov2(d, s) = ins else { continue };
            // println!("d: {}, s: {}", d.index(), s.index());
            let (mut d, mut s) = (
                dsu.find(d.index() as i32),
                dsu.find(s.index() as i32)
            );
            // println!("ad: {}, as: {}", d, s);
            if d == s { continue };
            let c = amat[d as usize].contains(&(s as usize));
            if c { continue };

            // If one register is pre-colored, change the other
            // To match it.
            if d < GPRS as i32 && s < GPRS as i32 {
                continue;
            } else if d < GPRS as i32 && s >= GPRS as i32 {
                dsu.mergef(d, s);
            } else if d >= GPRS as i32 && s < GPRS as i32 {
                dsu.mergef(s, d);
            } else {
                dsu.merge(d, s);
            }
            // If s -> d, swap the two.
            if dsu.find(s) == d {
                swap(&mut d, &mut s);
            }
            // Map (d -> s)
            let temp = std::mem::take(&mut alist[d as usize]);
            for node in &temp {
                amat[s as usize].insert(*node);
                amat[*node].insert(s as usize);
            }
            alist[s as usize].extend(temp);
            f.insert(d, s);
        }
        if f.len() == 0 { break }
        rewrite(live, |r: Reg| {
            match f.get(&(r.index() as i32)) {
                None    => return r,
                Some(i) => return Reg::from(*i as u32)
            };
        });
        // The coloring algorithm won't work because
        // We haven't updated amat and alist properly.
        // Rebuild the graph.
        (*amat, *alist) = build_graph(
            alist.len() as u32, live
        );
    }
}

fn color_graph(alist: AdjList) -> Vec<usize> {
    // If no coloring was found, recolor.
    let (mut degrees, mut stk);
    loop {
        let mut nodes = Vec::new();
        let mut stacked = vec![false; alist.len()];
        degrees = vec![0; alist.len()];
        stk     = Vec::new();
        for i in GPRS..alist.len() {
            nodes.push(Node {
                deg: alist[i].len() as u32,
                pos: i as u32
            });
            degrees[i] = alist[i].len() as u32;
        }

        while let Some(Node { deg, pos }) = nodes.pop() {
            if stacked[pos as usize] { continue }
            // We've updated it multiple times, and this is an old version.
            if degrees[pos as usize] < deg { continue }
            if deg >= GPRS as u32 { continue }
            for nbr in &alist[pos as usize] {
                // We don't update the adjlist
                // Hence, we need this check.
                if stacked[*nbr] { continue }
                if degrees[*nbr] == 0 { continue }
                degrees[*nbr] -= 1;
                nodes.push(Node {
                    deg: degrees[*nbr],
                    pos: *nbr as u32
                });
            }
            stk.push(pos);
            stacked[pos as usize] = true;
        }
        if stk.len() != (alist.len() - GPRS) {
            // Spill Logic.
        }
        break;
    }

    let mut legal  = vec![[true; GPRS]; degrees.len()];
    let mut colors = vec![None; degrees.len()];
    // Pre-Color Nodes.
    for i in 0..GPRS {
        colors[i as usize] = Some(i);
        for nbr in &alist[i as usize] {
            legal[*nbr][i] = false;
        }
    }

    // Determine the Colors...
    while let Some(idx) = stk.pop() {
        for (i, l) in legal[idx as usize].iter().enumerate() {
            if !l { continue }
            colors[idx as usize] = Some(i);
            for nbr in &alist[idx as usize] {
                legal[*nbr][i] = false;
            }
            break;
        }
    }
    // Ensure every ID has a color.
    return colors
        .into_iter()
        .map(|x| x.unwrap())
        .collect();
}

pub fn build_graph(
        nids: u32,
        live: &Vec<(AA, Vec<bool>, Vec<bool>)>
    ) -> (AdjMatrix, AdjList) {
    // We need both because we access the graph
    // sequentially and randomly.
    let mut amat = vec![
        HashSet::new();
        (nids as usize) + GPRS
    ];
    let mut alist = vec![
        Vec::new();
        (nids as usize) + GPRS
    ];
    // Literally the Chaitin Graph Building Algo.
    // 1. We store counts because there WILL be duplicates
    // because of graph coalescing.
    // 2. This has been cleverly designed to allow dynamically
    // recomputing the liveness, in linear time!
    use AA::*;
    let mut conflicts = HashMap::new();
    for (asm, defdead, usedead) in live {
        if let BB(v) = asm.clone() {
            conflicts = HashMap::new();
            for reg in v {
                match conflicts.get(&reg.index()) {
                    None    => conflicts.insert(reg.index(), 1),
                    Some(i) => conflicts.insert(reg.index(), *i + 1)
                };
            }
        } else {
            let (defs, uses) = asm.defuse();
            // If the register dies after this use, it doesn't produce a conflict.
            // The count is important here, if the count is non-zero, that means
            // That a duplicate of this register has been inserted (prbly bc coalescing)
            // And hence, the register is still live because the duplicate is in use.
            for (reg, dead) in uses.iter().zip(usedead.iter()) {
                // println!("use: reg - {}, dead - {}", reg, dead);
                if !*dead { continue }
                match conflicts.get(&reg.index()).unwrap() {
                    1  => conflicts.remove(&reg.index()),
                    i  => conflicts.insert(reg.index(), *i - 1)
                };
            }
            for (reg, dead) in defs.iter().zip(defdead.iter()) {
                // println!("def: reg - {}, dead - {}", reg, dead);
                // Add edges between everything which conflicts with this definition.
                for (key, _) in &conflicts {
                    if amat[reg.index()].contains(key) { continue }
                    amat[reg.index()].insert(*key); 
                    amat[*key].insert(reg.index());
                    alist[reg.index()].push(*key);
                    alist[*key].push(reg.index());
                }
                if *dead { continue }
                // We've redefined the variable (and it's used somewhere)
                // Hence, it could conflict with stuff in the future.
                match conflicts.get(&reg.index()) {
                    None    => conflicts.insert(reg.index(), 1),
                    Some(i) => conflicts.insert(reg.index(), *i + 1)
                };
            }
        }
    }
    use Reg as R;
    // Prevent overwriting SP, RZR, PC
    for illegal in vec![R::SP, R::RZR, R::PC] {
        for reg in GPRS..alist.len() {
            if amat[illegal.index()].contains(&reg) { continue }
            amat[illegal.index()].insert(reg); 
            amat[reg].insert(illegal.index());
            alist[illegal.index()].push(reg);
            alist[reg].push(illegal.index());
        }
    }
    return (amat, alist)
}

fn rewrite(
        live: &mut Vec<(AA, Vec<bool>, Vec<bool>)>,
        c: impl Fn(Reg) -> Reg
    ) {
    use AA::*;
    for (asm, _, _) in live {
        *asm = match asm.clone() {
            Label(l)           => Label(l),
            Mov1(d, s)         => Mov1(c(d), s),
            Mov2(d, s)         => Mov2(c(d), c(s)),
            Add1(d, l, r)      => Add1(c(d), c(l), r),
            Add2(d, l, r)      => Add2(c(d), c(l), c(r)),
            Sub1(d, l, r)      => Sub1(c(d), c(l), r),
            Sub2(d, l, r)      => Sub2(c(d), c(l), c(r)),
            Neg1(d, s)         => Neg1(c(d), s),
            Neg2(d, s)         => Neg2(c(d), c(s)),
            SMAddL(d, l, m, r) => SMAddL(c(d), c(l), c(m), c(r)),
            SMNegL(d, l, r)    => SMNegL(c(d), c(l), c(r)),
            SMSubL(d, l, m, r) => SMSubL(c(d), c(l), c(m), c(r)),
            SMulL(d, l, r)     => SMulL(c(d), c(l), c(r)),
            SDiv(d, l, r)      => SDiv(c(d), c(l), c(r)),
            And1(d, l, r)      => And1(c(d),  c(l), r),
            And2(d, l, r)      => And2(c(d),  c(l), c(r)),
            Or1(d, l, r)       => Or1(c(d), c(l), r),
            Or2(d, l, r)       => Or2(c(d), c(l), c(r)),
            Mvn1(d, s)         => Mvn1(c(d), s),
            Mvn2(d, s)         => Mvn2(c(d), c(s)),
            B(l)               => B(l),
            BL(l)              => BL(l),
            CBZ(l)             => CBZ(l),
            CBNZ(l)            => CBNZ(l),
            CMP1(d, s)         => CMP1(c(d), s),
            CMP2(d, s)         => CMP2(c(d), c(s)),
            CSET(d, s)         => CSET(c(d), s),
            LDR1(d, l, r)      => LDR1(c(d), c(l), r),
            LDR2(d, s)         => LDR2(c(d), c(s)),
            STR1(d, l, r)      => STR1(c(d), c(l), r),
            STR2(d, s)         => STR2(c(d), c(s)),
            Ret                => Ret,
            BB(v)              => BB(v.iter().map(|r| c(*r)).collect())
        };
    }
}

// Utilities...
pub fn print_graph(alist: AdjList) {
    println!("digraph interference_graph {{");
    let mut added = vec![
        false; alist.len() + GPRS as usize
    ];
    for (idx, v) in alist.into_iter().enumerate() {
        if v.len() == 0 { continue }
        println!("    node{} [label=\"{}\"]",
            idx, Reg::from(idx as u32)
        );
        for nbr in v {
            if added[idx] { continue }
            println!("    node{} -> node{} [dir=both];", idx, nbr);
            added[idx] = true;
            added[nbr] = true;
        }
    }
    println!("}}");
}