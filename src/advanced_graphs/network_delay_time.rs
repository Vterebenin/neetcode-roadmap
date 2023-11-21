use crate::utils::print_pass;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let adjacency_list = {
        let mut res = vec![vec![]; n + 1];
        for time in times {
            let u = time[0] as usize;
            let v = time[1] as usize;
            let w = time[2];
            res[u].push((v, w));
        }
        res
    };

    // dijkstra
    let mut distances = vec![std::i32::MAX; n + 1];
    distances[k] = 0;
    let mut visited = vec![false; n + 1];
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), k));
    while let Some((Reverse(distance), u)) = pq.pop() {
        if visited[u] {
            continue;
        }
        for &(v, w) in &adjacency_list[u] {
            let new_distance = w + distance;
            if new_distance < distances[v] {
                distances[v] = new_distance;
                pq.push((Reverse(new_distance), v));
            }
        }
        visited[u] = true;
    }
    if !visited[1..].iter().all(|&a| a) {
        return -1;
    }
    *distances[1..].iter().max().unwrap()
}

const NAME: &str = "network-delay-time";


pub fn main() {
    let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
    let n = 4;
    let k = 2;
    assert_eq!(network_delay_time(times, n, k), 2);
    let times = vec![vec![1, 2, 1], vec![2, 1, 3]];
    let n = 2;
    let k = 2;
    assert_eq!(network_delay_time(times, n, k), 3);
    let times = vec![vec![1, 2, 1], vec![2, 3, 2], vec![1, 3, 4]];
    let n = 3;
    let k = 1;
    assert_eq!(network_delay_time(times, n, k), 3);
    let times = vec![vec![1, 2, 1], vec![2, 3, 2], vec![1, 3, 1]];
    let n = 3;
    let k = 2;
    assert_eq!(network_delay_time(times, n, k), -1);

    print_pass(NAME)
}
