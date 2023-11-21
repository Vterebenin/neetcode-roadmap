use crate::utils::print_pass;

const NAME: &str = "max-area-of-island";


pub struct Point {
    x: i32,
    y: i32,
}
static DIR: [[i32; 2]; 4] = [
    [-1, 0],
    [1, 0],     
    [0, -1],
    [0, 1],
];
pub fn walk(grid: &Vec<Vec<i32>>, seen: &mut Vec<Vec<bool>>, curr: Point, area_counter: &mut i32) {
    if curr.y < 0 || curr.y as usize >= grid.len() || curr.x < 0 || curr.x as usize >= grid[0].len() {
        return;
    }
    if seen[curr.y as usize][curr.x as usize] {
        return;
    }

    seen[curr.y as usize][curr.x as usize] = true;
    if grid[curr.y as usize][curr.x as usize] != 1 {
        return;
    }
    *area_counter += 1;
    for i in 0..DIR.len() {
        if let [x, y] = DIR[i][..] {
            walk(grid, seen, Point { x: curr.x + x, y: curr.y + y }, area_counter);
        }
    }
}
pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut max_area = 0;
    let mut seen = vec![vec![false; grid[0].len()];grid.len()];

    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char.to_owned() == 1 && !seen[y][x] {
                let mut area_counter = 0;
                let (x, y) = (x as i32, y as i32);
                walk(&grid, &mut seen, Point { x, y }, &mut area_counter);
                if area_counter > max_area { 
                    max_area = area_counter.to_owned();
                }
            }
        }
    }
    max_area
}

pub fn main() {
    let grid: Vec<Vec<i32>> = vec![ 
        vec![1,1,1,1,0], 
        vec![1,1,0,1,0], 
        vec![1,1,0,0,0], 
        vec![0,0,0,0,0], 
    ];
    let max_area: i32 = 9;
    assert_eq!(max_area_of_island(grid), max_area);
    let grid: Vec<Vec<i32>> = vec![ 
        vec![1,1,0,0,0], 
        vec![1,1,0,0,0], 
        vec![0,0,1,0,0], 
        vec![0,0,0,1,1] 
    ];
    let max_area: i32 = 4;
    assert_eq!(max_area_of_island(grid), max_area);
    print_pass(NAME)
}
