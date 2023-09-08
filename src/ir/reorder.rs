use super::cfg::CFG;
pub fn reorder(cfg: &CFG) -> Vec<usize> {
    fn reorder_function(start: usize, count: &mut i32, order: &mut [i32], cfg: &CFG) {
        *count += 1;
        order[start] = *count;
        let node = &cfg.nodes[start as usize];
        if let Some(t) = node.t {
            if order[t as usize] < 0 { 
                reorder_function(t, count, order, cfg);
            }
        }
        if let Some(f) = node.f {
            if order[f as usize] < 0 { 
                reorder_function(f, count, order, cfg);
            }
        }
    }
    let mut count = -1;
    let mut order = vec![-1; cfg.nodes.len()];
    for start in &cfg.starts {
        reorder_function(
            *start,
            &mut count,
            &mut order,
            &cfg
        );
    }
    return order.into_iter()
        .map(|x| x as usize)
        .collect();
}