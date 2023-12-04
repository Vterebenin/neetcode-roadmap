use crate::utils::print_pass;

pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let n = n as usize;
    let src = src as usize;
    let dst = dst as usize;
    let adjacency_list = {
        let mut res = vec![vec![]; n];
        for time in flights {
            let u = time[0] as usize;
            let v = time[1] as usize;
            let w = time[2];
            res[u].push((v, w));
        }
        res
    };

    let mut dists = vec![i32::MAX; n];
    dists[src] = 0;

    // For each vertex, apply relaxation for all the edges
    for _ in 0..=k {
        let mut dists2 = dists.clone();
        for (from, edges) in adjacency_list.iter().enumerate() {
            if dists[from] != i32::MAX {
                for (to, cost) in edges {
                    dists2[*to] = dists2[*to].min(dists[from] + cost);
                }
            }
        }
        dists = dists2;
    }
    
    if dists[dst] == std::i32::MAX {
        return -1;
    }
    dists[dst]
}

const NAME: &str = "cheapest-flights-within-k-stops";

pub fn main() {
    let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![2, 0, 100], vec![1, 3, 600], vec![2,3,200]];
    let n = 4;
    let src = 0;
    let dst = 3;
    let k = 1;
    assert_eq!(find_cheapest_price(n, flights, src, dst, k), 700);

    print_pass(NAME)
}
