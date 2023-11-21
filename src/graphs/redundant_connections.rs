use crate::utils::print_pass;

const NAME: &str = "redundant-connection";


#[derive(Debug)]
struct UnionFind {
    parent: Vec<i32>,
}

impl UnionFind {
    fn new(n: i32) -> Self {
        UnionFind { parent: (0..n).collect() }
    }

    fn find(&mut self, x: i32) -> i32 {
        if self.parent[x as usize] != x {
            self.parent[x as usize] = self.find(self.parent[x as usize]);
        }
        self.parent[x as usize]
    }

    fn union(&mut self, x: i32, y: i32) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_x as usize] = root_y;
        }
    }
}

fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len();
    let mut uf = UnionFind::new(n as i32);

    for edge in &edges {
        let u = edge[0] - 1;
        let v = edge[1] - 1;
        if uf.find(u) == uf.find(v) {
            return vec![u as i32 + 1, v as i32 + 1];
        } else {
            uf.union(u, v);
        }
    }

    vec![]
}

pub fn main() {
    let edges = vec![vec![1,2],vec![1,3],vec![2,3]];
    assert_eq!(find_redundant_connection(edges), vec![2, 3]);
    let edges = vec![vec![1,2],vec![2,3],vec![3,4],vec![1,4],vec![1,5]];
    assert_eq!(find_redundant_connection(edges), vec![1, 4]);
    print_pass(NAME)
}
