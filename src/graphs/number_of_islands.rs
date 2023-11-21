use crate::utils::print_pass;

const NAME: &str = "number-of-islands";


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
pub fn walk(grid: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>, curr: Point) {
    if curr.y < 0 || curr.y as usize >= grid.len() || curr.x < 0 || curr.x as usize >= grid[0].len() {
        return;
    }
    if seen[curr.y as usize][curr.x as usize] {
        return;
    }

    seen[curr.y as usize][curr.x as usize] = true;
    if grid[curr.y as usize][curr.x as usize] != '1' {
        return;
    }

    for i in 0..DIR.len() {
        if let [x, y] = DIR[i][..] {
            walk(grid, seen, Point { x: curr.x + x, y: curr.y + y });        
        }
    }
}
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut counter = 0;
    let mut seen = vec![vec![false; grid[0].len()];grid.len()];

    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char.to_owned() == '1' && !seen[y][x] {
                let (x, y) = (x as i32, y as i32);
                walk(&grid, &mut seen, Point { x, y });
                counter += 1;
            }
        }
    }
    counter
}

pub fn main() {
    let grid: Vec<Vec<char>> = vec![ 
        vec!['1','1','1','1','0'], 
        vec!['1','1','0','1','0'], 
        vec!['1','1','0','0','0'], 
        vec!['0','0','0','0','0'], 
    ];
    let number_of_islands: i32 = 1;
    assert_eq!(num_islands(grid), number_of_islands);
    let grid: Vec<Vec<char>> = vec![ 
        vec!['1','1','0','0','0'], 
        vec!['1','1','0','0','0'], 
        vec!['0','0','1','0','0'], 
        vec!['0','0','0','1','1'] 
    ];
    let number_of_islands: i32 = 3;
    assert_eq!(num_islands(grid), number_of_islands);
    print_pass(NAME)
}
