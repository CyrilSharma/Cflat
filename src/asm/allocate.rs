use crate::registry::Registry;

use super::asm::*;
use super::liveness::Liveness;

use std::cmp::Ordering;
use std::collections::{HashSet, BTreeSet, VecDeque};
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    legal: u32,
    deg:   u32,
    pos:   u32,
}

// Binary Heap is Largest to Smallest by default.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Select the Node with the smallest # of legal options,
        // Followed by Nodes with the highest Degree (fail fast)
        other.legal.cmp(&self.legal)
            .then_with(|| self.deg.cmp(&other.deg))
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Allocator;
impl Allocator {
    pub fn allocate(r: &mut Registry, asm: Vec<AA>, live: Liveness) {
        // The options remaining.
        let mut legal: Vec<BTreeSet<Reg>> = vec![
            BTreeSet::new(); 
            r.nids as usize
        ];
        // Maps node-idx to assigned Register.
        let mut registers: Vec<Option<Reg>> = vec![
            None;
            r.nids as usize
        ];
        let mut degrees: Vec<u32> = vec![0; r.nids as usize];
        // HashSet for efficient Deletion.
        let mut graph = vec![
            HashSet::<Reg>::new();
            r.nids as usize
        ];
        // Compute the Graph.
        for entry in live.lin {
            for i in 0..entry.len() {
                let s = entry[i];
                for j in (i+1)..entry.len() {
                    let d = entry[j];
                    graph[s.index()].insert(d); 
                    graph[d.index()].insert(s);
                    degrees[s.index()] += 1;
                    degrees[d.index()] += 1;
                }
            }
        }

        // The nodes whose color remains unassigned.
        let mut nodes: BinaryHeap<Node> = BinaryHeap::new();
        for ind in 0..(r.nids as usize) {
            let node = Node {
                legal:  legal[ind].len() as u32,
                deg:    degrees[ind],
                pos:    ind as u32
            };
            nodes.push(node); 
        }

        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut choose = |idx: usize, reg: Reg| {
            // Slight performance overhead. Use Cell.
            let mut erase: Vec<usize> = vec![];
            for nbr in &graph[idx] { erase.push(nbr.index()) }
            for e in erase {
                if let Some(r) = registers[e] {
                    if r.index() != idx { continue }
                    feasible = false;
                    return;
                }
                legal[e].remove(&reg);
                let len = legal[e].len();
                if len == 0 {
                    feasible = false;
                    return;
                } else if len == 1 {
                    queue.push(e);
                }
            }
        };

        let propogate = || {
            loop {
                let Some(idx) = queue.pop_front() else { break };
                if let Some(_) = registers[idx] { continue }
                if legal[idx].len() == 0 { break }
                else if legal[idx].len() != 1 { continue }
                let choice = legal[idx].pop_first().unwrap();
                let (feasible, enqueue) = choose(idx, choice);
                if !feasible { break }
                for e in enqueue { queue.push_back(e) }
            }
        };
        
        todo!()
    }
    pub fn build() {}
}