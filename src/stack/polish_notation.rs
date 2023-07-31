use crate::utils::print_pass;

const NAME: &str = "evaluate-reverse-polish-notation";
const LINK: &str = "https://leetcode.com/problems/evaluate-reverse-polish-notation/";

fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::new();
    for s in tokens.iter() {
        if let Ok(x) = s.parse::<i32>() {
            stack.push(x);
            continue;
        }
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.push(match s.as_str() {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            what => panic!("What's this '{}'", what),
        });
    }
    stack.pop().unwrap()
}

pub fn main() {
    let notation: Vec<String> = vec![
        String::from("2"), 
        String::from("1"), 
        String::from("+"), 
        String::from("3"), 
        String::from("*")
    ];
    assert_eq!(eval_rpn(notation), 9);
    print_pass(NAME, LINK)
}
