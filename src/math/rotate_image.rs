use crate::utils::print_pass;

const NAME: &str = "rotate-image";


pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    *matrix = matrix[0]
        .iter()
        .enumerate()
        .map(|(index, _val)| {
            return matrix.iter().map(move |row| row[index]).rev().collect::<Vec<i32>>();
        })
        .collect::<Vec<Vec<i32>>>();
}

pub fn main() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    rotate(&mut matrix);
    assert_eq!(matrix.clone(), vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    print_pass(NAME)
}
