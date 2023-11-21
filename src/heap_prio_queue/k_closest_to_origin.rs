use crate::utils::print_pass;
use std::collections::BinaryHeap;

const NAME: &str = "k-closest-points-to-origin";



pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    let mut pq = BinaryHeap::with_capacity(k);
    for p in points.iter() {
        let d = p[0] * p[0] + p[1] * p[1];
        pq.push((d, vec![p[0], p[1]]));
        if pq.len() > k {
            pq.pop();
        }
    }
    pq.into_iter().map(|(_, p)| p).collect()
}

pub fn main() {
    let points = vec![
        vec![2,2],
        vec![2,2],
        vec![2,2],
        vec![2,2],
        vec![2,2],
        vec![2,2],
        vec![1,1]
    ];
    let k = 1;
    assert_eq!(k_closest(points.clone(), k), [[1, 1]]);
    let points = vec![
        vec![2,2],
        vec![2,2],
        vec![3,3],
        vec![2,-2],
        vec![1,1]
    ];
    let k = 4;
    assert_eq!(k_closest(points.clone(), k), [[2,2],[2,2],[1,1],[2,-2]]);
    print_pass(NAME);
}
