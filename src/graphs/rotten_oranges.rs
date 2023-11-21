use std::collections::VecDeque;

use crate::utils::print_pass;

const NAME: &str = "rotten-oranges";


pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut q = VecDeque::new();  
    let mut time = 0;
    let mut fresh = 0;

    let rows = grid.len();
    let cols = grid[0].len();
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 1 {
                fresh += 1;
            } 
            if grid[r][c] == 2 {
                q.push_back((r, c))
            }
        }
    }

    let dir: Vec<Vec<i32>> = vec![vec![0, 1], vec![0, -1], vec![1, 0], vec![-1, 0]];
    while !q.is_empty() && fresh > 0 {
        for _i in 0..q.len() {
            if let Some((r, c)) = q.pop_front() {
                for d in dir.iter() {
                    let row = d[0] + r as i32;
                    let col = d[1] + c as i32;

                    if row < 0 || row as usize == rows || 
                        col < 0 || col as usize == cols || grid[row as usize][col as usize] != 1 {
                        continue;
                    }
                    grid[row as usize][col as usize] = 2;
                    q.push_back((row as usize, col as usize));
                    fresh -= 1;
                }
            }
        }
        time += 1;
    }
    if fresh == 0 {
        return time;
    } else {
        return -1;
    }
}

pub fn main() {
    let grid: Vec<Vec<i32>> = vec![ 
        vec![2, 1, 1], 
        vec![1, 1, 0], 
        vec![0, 1, 1], 
    ];
    let max_area: i32 = 4;
    assert_eq!(oranges_rotting(grid), max_area);
    let grid: Vec<Vec<i32>> = vec![ 
        vec![0, 2], 
    ];
    let max_area: i32 = 0;
    assert_eq!(oranges_rotting(grid), max_area);
    print_pass(NAME)
}
