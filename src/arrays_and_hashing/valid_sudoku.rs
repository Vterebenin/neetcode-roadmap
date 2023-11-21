use crate::utils::print_pass;

const NAME: &str = "valid-anagram";


const SIZE: usize = 9;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for i in 0..SIZE {
        let mut row_set = vec![false; SIZE];
        let mut col_set = vec![false; SIZE];
        for j in 0..SIZE {
            let item = board[i][j];
            if item != '.' {
                let item = item.to_digit(10).unwrap() as usize - 1;
                if row_set[item] {
                    return false;
                }
                row_set[item] = true;
            }
        }
        for j in 0..SIZE {
            let item = board[j][i];
            if item != '.' {
                let item = item.to_digit(10).unwrap() as usize - 1;
                if col_set[item] {
                    return false;
                }
                col_set[item] = true;
            }
        }
    }
    for i in 0..SIZE / 3 {
        for j in 0..SIZE / 3 {
            let mut subgrid_set = [false; SIZE];
            for x in 0..3 {
                for y in 0..3 {
                    let item = board[i * 3 + x][j * 3 + y];
                    if item != '.' {
                        let num = item.to_digit(10).unwrap() as usize - 1;
                        if subgrid_set[num] {
                            return false;
                        }
                        subgrid_set[num] = true;
                    }
                }
            }
        }
    }

    true
}


pub fn main() {
    let board = vec![
        vec!['8','3','.','.','7','.','.','.','.']
            ,vec!['6','.','.','1','9','5','.','.','.']
            ,vec!['.','9','8','.','.','.','.','6','.']
            ,vec!['8','.','.','.','6','.','.','.','3']
            ,vec!['4','.','.','8','.','3','.','.','1']
            ,vec!['7','.','.','.','2','.','.','.','6']
            ,vec!['.','6','.','.','.','.','2','8','.']
            ,vec!['.','.','.','4','1','9','.','.','5']
            ,vec!['.','.','.','.','8','.','.','7','9']
    ];
    
    assert!(!is_valid_sudoku(board));
    let board = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9']
    ];
    assert!(is_valid_sudoku(board));
    print_pass(NAME);
}
