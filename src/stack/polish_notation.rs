use crate::utils::print_pass;

const NAME: &str = "evaluate-reverse-polish-notation";
const LINK: &str = "https://leetcode.com/problems/evaluate-reverse-polish-notation/";

fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];
    for token in tokens.iter() {
        match token.as_str() {
            "+"|"-"|"*"|"/" => {
                if stack.len() < 2 { continue; }
                let last: i32 = stack.pop().unwrap();
                let previous: i32 = stack.pop().unwrap();
                let result = match token.as_str() {
                    "+" => last + previous,
                    "-" => previous - last,
                    "*" => last * previous,
                    "/" => previous / last,
                    _ => 0,
                };
                stack.push(result);
            },
            _ => stack.push(token.parse::<i32>().unwrap()),
        }
    };
    *stack.first().unwrap()
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
