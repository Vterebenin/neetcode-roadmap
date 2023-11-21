use crate::utils::print_pass;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
struct Graph {
    nodes: HashMap<i32, HashMap<i32, i32>>, // Node, Neighbor, Cost
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Vertex {
    id: i32,
    distance: i32,
}
//
// Implement a custom ordering for the BinaryHeap.
impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance) // Reverse ordering
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: i32) {
        self.nodes.entry(node).or_insert(HashMap::new());
    }

    fn add_edge(&mut self, node1: i32, node2: i32, cost: i32) {
        self.nodes.get_mut(&node1).unwrap().insert(node2, cost);
    }
}

fn dijkstra(graph: &Graph, start: i32) -> HashMap<i32, i32> {
    let graph = &graph.nodes;
    let mut distances: HashMap<i32, i32> = graph.keys().map(|&k| (k, i32::MAX)).collect();
    distances.insert(start, 0);

    let mut priority_queue: BinaryHeap<Vertex> = BinaryHeap::new();
    priority_queue.push(Vertex {
        id: start,
        distance: 0,
    });

    while let Some(current_vertex) = priority_queue.pop() {
        let current_id = current_vertex.id;
        let current_distance = current_vertex.distance;

        if current_distance > distances[&current_id] {
            continue;
        }

        if let Some(neighbors) = graph.get(&current_id) {
            for (&neighbor, &weight) in neighbors {
                let new_distance = current_distance + weight;

                if new_distance < distances[&neighbor] {
                    distances.insert(neighbor, new_distance);
                    priority_queue.push(Vertex {
                        id: neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }
    distances
}

pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let mut graph = Graph::new();
    for time in times {
        let (node1, node2, cost) = (time[0], time[1], time[2]);
        graph.add_node(node1);
        graph.add_node(node2);
        graph.add_edge(node1, node2, cost);
    }
    let dist = dijkstra(&graph, k);
    let mut max = 0;
    if dist.len() != n as usize {
        return -1;
    }
    for (_, cost) in dist {
        if cost == i32::MAX {
            return -1;
        }
        if cost != i32::MAX && cost != 0 {
            max = cost.max(max);
        }
    }
    if max == 0 {
        return -1;
    }
    max
}
const NAME: &str = "network-delay-time";
const LINK: &str = "https://leetcode.com/problems/network-delay-time/";

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
    assert_eq!(network_delay_time(times, n, k), 3);

    print_pass(NAME, LINK)
}
