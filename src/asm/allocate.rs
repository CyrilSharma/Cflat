// See https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.452.8606&rep=rep1&type=pdf
use crate::registry::Registry;

use super::asm::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::BinaryHeap;


type AdjMatrix = Vec<HashSet<usize>>;
type AdjList   = Vec<Vec<usize>>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    deg:   u32,
    pos:   u32,
}

// Binary Heap is Largest to Smallest by default.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Select the Node with the smallest # of legal options,
        // Followed by Nodes with the highest Degree (fail fast)
        other.deg.cmp(&self.deg)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn allocate(
        r: &mut Registry,
        asm:  Vec<AA>,
        defs: Vec<Vec<Reg>>,
        live: Vec<Vec<Reg>>,
    ) -> Vec<AA> {
    let (_, alist) = build_graph(r.nids, defs, live);
    let colors = color_graph(alist);
    return color_asm(asm, colors);
}

fn color_graph(alist: AdjList) -> Vec<usize> {
    let mut again   = true;
    let mut stk     = Vec::new();
    let mut spill   = Vec::new();
    let mut degrees = vec![0; alist.len()];
    let mut legal   = vec![[true; GPRS]; degrees.len()];
    let mut colors  = vec![None; degrees.len()];
    while again {
        let mut nodes = BinaryHeap::new();
        for i in 0..alist.len() {
            nodes.push(Node {
                deg: alist[i].len() as u32,
                pos: i as u32
            });
            degrees[i] = alist[i].len() as u32;
        }
        'l: while let Some(Node { deg, pos }) = nodes.pop() {
            if degrees[pos as usize] < deg { continue }
            if deg >= GPRS as u32 {
                spill.push(pos);
                break 'l;
            }
            for nbr in &alist[pos as usize] {
                if degrees[*nbr] == 0 { continue }
                if pos < GPRS as u32 {
                    legal[*nbr][pos as usize] = false
                }
                degrees[*nbr] -= 1;
                nodes.push(Node {
                    deg: degrees[*nbr],
                    pos: *nbr as u32
                });
            }
            if pos < GPRS as u32 {
                // Don't push as we've already colored it.
                colors[pos as usize] = Some(pos as usize);
                continue;
            }
            stk.push(pos)
        }
        if spill.len() != 0 {
            todo!();
        } else {
            again = false;
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

pub fn print_graph(
        nids: u32,
        defs: &Vec<Vec<Reg>>,
        live: &Vec<Vec<Reg>>
    ) {
    let (_, alist) = build_graph(
        nids, defs.clone(), live.clone()
    );
    println!("digraph interference_graph {{");
    let mut added = vec![
        HashSet::new();
        nids as usize + GPRS as usize
    ];
    for (idx, v) in alist.into_iter().enumerate() {
        if v.len() == 0 { continue }
        println!("    node{} [label=\"{}\"]",
            idx, Reg::from(idx as u32)
        );
        for nbr in v {
            if added[idx].contains(&nbr) { continue }
            println!("    node{} -> node{} [dir=both];", idx, nbr);
            added[idx].insert(nbr);
            added[nbr].insert(idx);
        }
    }
    println!("}}");
}

fn build_graph(
        nids: u32,
        defs: Vec<Vec<Reg>>,
        live: Vec<Vec<Reg>>
    ) -> (AdjMatrix, AdjList) {
    // HashSet for efficient Deletion.
    let mut amat = vec![
        HashSet::new();
        (nids as usize) + GPRS
    ];
    let mut alist = vec![
        Vec::new();
        (nids as usize) + GPRS
    ];
    // Compute the Graph.
    for i in 0..defs.len() {
        for d in &defs[i] {
            for l in &live[i] {
                let c = amat[d.index()].contains(&l.index());
                if c { continue }
                alist[d.index()].push(l.index()); 
                alist[l.index()].push(d.index());
                amat[d.index()].insert(l.index());
                amat[l.index()].insert(d.index());
            }
        }
    }
    return (amat, alist)
}

fn color_asm(asm: Vec<AA>, colors: Vec<usize>) -> Vec<AA> {
    use AA::*;
    use Reg::*;
    let c = |r: Reg| {
        return R(colors[r.index()] as u8)
    };
    let mut res = Vec::new();
    for a in asm {
        res.push(match a {
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
            SMSubL(d, l, m, r) => SMSubL(c(d), c(l), c(r), c(r)),
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
            Ret                => Ret
        });
    }
    return res;
}