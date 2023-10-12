use crate::utils::print_pass;

const NAME: &str = "spiral-matrix";
const LINK: &str = "https://leetcode.com/problems/spiral-matrix/";

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let dirs: Vec<(i32, i32)> = vec![
        (1, 0), // right
        (0, 1), // down
        (-1, 0), // left
        (0, -1), // up
    ];
    let mut response = vec![];
    let mut i = 0 as i32;
    let mut j = 0 as i32;
    let mut start_x = 0;
    let mut end_x = matrix[0].len() as i32 - 1;
    let mut start_y = 0;
    let mut end_y = matrix.len() as i32 - 1;
    let size = matrix.len() * matrix[0].len();
    while response.len() < matrix.len() * matrix[0].len() {
        for (x, y) in &dirs {
            let x = x.to_owned();
            let y = y.to_owned();
            if start_x > end_x || start_y > end_y {
                return response;
            }
            if x != 0 {
                i = if x == 1 { start_y } else if x == -1 { end_y } else { i };
                while j >= start_x && j <= end_x && response.len() < size {
                    response.push(matrix[i as usize][j as usize]);
                    j += x;
                }
                if x > 0 {
                    start_y += 1;
                    i += 1;
                } else {
                    end_y -= 1;
                    i -= 1;
                }
            } else if y != 0 {
                j = if y == 1 { end_x } else if y == -1 { start_x } else { j };
                while i >= start_y && i <= end_y && response.len() < size {
                    response.push(matrix[i as usize][j as usize]);
                    i += y;
                }
                if y > 0 {
                    end_x -= 1;
                    j -= 1;
                } else {
                    start_x += 1;
                    j += 1;
                }
            }
        }
    }
    response
}

pub fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let answer = vec![1,2,3,6,9,8,7,4,5];
    assert_eq!(spiral_order(matrix), answer);
    let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
    let answer = vec![1,2,3,4,8,12,11,10,9,5,6,7];
    assert_eq!(spiral_order(matrix), answer);
    print_pass(NAME, LINK)
}
