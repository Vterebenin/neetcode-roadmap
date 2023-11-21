use crate::utils::print_pass;

const NAME: &str = "unique-paths";


pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut table = vec![vec![0; n + 1]; m + 1];
    table[1][1] = 1;
    for i in 0..=m {
        for j in 0..=n {
            let row = i as usize;
            let col = j as usize; 
            let curr = table[row][col];
            if col + 1 <= n { table[row][col + 1] += curr; }
            if row + 1 <= m { table[row + 1][col] += curr; }
        }
    }
    table[m][n]
}

pub fn main() {
    assert_eq!(unique_paths(3, 7), 28);
    print_pass(NAME)
}
