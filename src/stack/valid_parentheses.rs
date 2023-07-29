use crate::utils::print_pass;

const NAME: &str = "valid-parentheses";
const LINK: &str = "https://leetcode.com/problems/valid-parentheses/";

fn valid_parentheses(s: String) -> bool {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        match c {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '}'|')'|']' if Some(c) != stack.pop() => return false,
            _ => (),
        }
    }
    stack.is_empty()
}

pub fn main() {
    let str = String::from("()[]{}");
    assert!(valid_parentheses(str));
    let str = String::from("(]");
    assert!(!valid_parentheses(str));
    print_pass(NAME, LINK)
}
