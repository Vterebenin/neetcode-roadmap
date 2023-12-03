use crate::utils::print_pass;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

fn manhattan_distance(p1: &[i32], p2: &[i32]) -> i32 {
    (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs()
}

pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let mut visited = vec![false; n];
    let mut heap_dict = HashMap::new();
    heap_dict.insert(0, 0);
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse((0, 0)));

    let mut mst_weight = 0;

    while let Some(Reverse((w, u))) = min_heap.pop() {
        if visited[u] || heap_dict[&u] < w {
            continue;
        }

        visited[u] = true;
        mst_weight += w;

        for v in 0..n {
            if !visited[v] {
                let new_distance = manhattan_distance(&points[u], &points[v]);  // Fix here
                if new_distance < *heap_dict.get(&v).unwrap_or(&i32::MAX) {
                    heap_dict.insert(v, new_distance);
                    min_heap.push(Reverse((new_distance, v)));
                }
            }
        }
    }

    mst_weight
}

const NAME: &str = "min-cost-to-connect-all-points";

pub fn main() {
    let points = vec![vec![0,0],vec![2,2],vec![3,10],vec![5,2],vec![7,0]];
    assert_eq!(min_cost_connect_points(points), 20);
    let points = vec![vec![3,12],vec![-2,5],vec![-4,1]];
    assert_eq!(min_cost_connect_points(points), 18);
    print_pass(NAME)
}
