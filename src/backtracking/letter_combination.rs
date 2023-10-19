use std::collections::HashMap;

use crate::utils::print_pass;

const NAME: &str = "combination-sum-ii";
const LINK: &str = "https://leetcode.com/problems/combination-sum/";


pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    let phone_items: HashMap<char, Vec<char>> = HashMap::from(
        [ 
        ('2', vec!['a', 'b', 'c']),
        ('3', vec!['d', 'e', 'f']),
        ('4', vec!['g', 'h', 'i']),
        ('5', vec!['j', 'k', 'l']),
        ('6', vec!['m', 'n', 'o']),
        ('7', vec!['p', 'q', 'r', 's']),
        ('8', vec!['t', 'u', 'v']),
        ('9', vec!['w', 'x', 'y', 'z'])
        ]
        );
    let chars: Vec<char> = digits.chars().collect();
    let mut result: Vec<String> = vec![];
    let mut buffer = String::new();
    let size = digits.len();
    dfs(chars, &mut result, &mut buffer, &phone_items, size);
    result
}
fn dfs(
    chars: Vec<char>, 
    result: &mut Vec<String>, 
    buffer: &mut String, 
    phone_items: &HashMap<char, Vec<char>>,
    size: usize,
) {
    if size == buffer.len() {
        result.push(buffer.clone());
        return;
    }
    if chars.is_empty() {
        return;
    }
    for i in 0..chars.len() {
        let items = phone_items.get(&chars[i]).unwrap();
        println!("{:?}", items.clone());
        for item in items {
            buffer.push(item.to_owned());
            println!("{:?}", buffer);
            dfs(chars[i + 1..].to_vec(), result, buffer, phone_items, size);
            buffer.pop();
        }
    }
}

pub fn main() {
    let digits: String = String::from("23");
    let answer = vec![
        "ad".to_string(),
        "ae".to_string(),
        "af".to_string(),
        "bd".to_string(),
        "be".to_string(),
        "bf".to_string(),
        "cd".to_string(),
        "ce".to_string(),
        "cf".to_string(),
    ];
    assert_eq!(letter_combinations(digits), answer);
    print_pass(NAME, LINK);
}

