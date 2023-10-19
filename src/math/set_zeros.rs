use crate::utils::print_pass;

const NAME: &str = "rotate-image";
const LINK: &str = "https://leetcode.com/problems/rotate-image/";

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let column_size = matrix.len();
    let row_size = matrix[0].len();
    let mut zero_rows = vec![false; column_size];
    let mut zero_columns = vec![false; row_size];
    for i in 0..column_size {
        for j in 0..row_size {
            if matrix[i][j] == 0 {
                zero_rows[i] = true;
                zero_columns[j] = true;
            }
        }
    }
    for i in 0..column_size {
        for j in 0..row_size {
            if zero_rows[i] || zero_columns[j] {
                matrix[i][j] = 0;
            }
        }
    }
}

pub fn main() {
    let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    set_zeroes(&mut matrix);
    assert_eq!(matrix.clone(), vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    let mut matrix = vec![vec![0, 1, 1, 0], vec![1, 1, 1, 1], vec![1, 1, 1, 1]];
    set_zeroes(&mut matrix);
    assert_eq!(matrix.clone(), vec![vec![0, 0, 0, 0], vec![0, 1, 1, 0], vec![0, 1, 1, 0]]);
    print_pass(NAME, LINK)
}
