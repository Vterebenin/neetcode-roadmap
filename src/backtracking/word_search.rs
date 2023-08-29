use crate::utils::print_pass;

const NAME: &str = "word-search";
const LINK: &str = "https://leetcode.com/problems/word-search/";

#[derive(Debug, Clone, Copy)]
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

pub fn walk(
    board: &Vec<Vec<char>>, 
    curr: Point, 
    word: &Vec<char>, 
    seen: &mut Vec<Vec<bool>>,
    curr_word: &mut Vec<char>, 
) -> bool {
    if curr.x < 0 || curr.x as usize >= board[0].len() 
        || curr.y < 0 || curr.y as usize >= board.len() {
        return false;
    }
    if seen[curr.y as usize][curr.x as usize] {
        return false;
    }
    let index = curr_word.len();
    if board[curr.y as usize][curr.x as usize] != word[index as usize] {
        return false;
    }
    curr_word.push(board[curr.y as usize][curr.x as usize]);
    if word.len() == curr_word.len() {
        return true;
    }
        
    seen[curr.y as usize][curr.x as usize] = true;
    
    for i in 0..DIR.len() {
        if let [x, y] = DIR[i][..] {
            if walk(&board, Point {
                x: curr.x + x,
                y: curr.y + y,
            }, &word, seen, curr_word) {
                return true;
            };
        }
    }
    curr_word.pop();
    seen[curr.y as usize][curr.x as usize] = false;
    false
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let mut curr_word: Vec<char> = vec![];
    let w = word.chars().collect::<Vec<char>>();
    for start_y in 0..board.len() {
        for start_x in 0..board[0].len() {
            let mut seen = vec![vec![false; board[0].len()];board.len()];
            if walk(&board, Point { x: start_x as i32, y: start_y as i32 }, &w, &mut seen, &mut curr_word) {
                return true;
            }
        }
    }
    false
}

pub fn main() {
    let board = vec![
        vec!['A','B','C','E'],
        vec!['S','F','C','S'],
        vec!['A','D','E','E']
    ];
    let word = String::from("ABCCED");
    assert_eq!(exist(board, word), true);
    let word = String::from("SSE");
    let board = vec![
        vec!['A','B','C','E'],
        vec!['S','F','C','S'],
        vec!['A','D','E','E']
    ];
    assert_eq!(exist(board, word), false);
    let word = String::from("ABCB");
    let board = vec![
        vec!['A','B','C','E'],
        vec!['S','F','C','S'],
        vec!['A','D','E','E']
    ];
    assert_eq!(exist(board, word), false);
    let word = String::from("AAB");
    let board = vec![
        vec!['C','A','A'],
        vec!['A','A','A'],
        vec!['B','C','D'],
    ];
    assert_eq!(exist(board, word), true);
    let word = String::from("ABCESEEEFS");
    let board = vec![
        vec!['A','B','C','E'],
        vec!['S','F','E','S'],
        vec!['A','D','E','E']
    ];
    assert_eq!(exist(board, word), true);
    print_pass(NAME, LINK)
}

