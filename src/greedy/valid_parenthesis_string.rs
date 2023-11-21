use crate::utils::print_pass;

const NAME: &str = "valid-parenthesis-string";


pub fn check_valid_string(s: String) -> bool {
    let mut min = 0;
    let mut max = 0;
    for c in s.chars() {
        match c {
            '*' => {
                min -= 1;
                max += 1;
            },
            '(' => {
                min += 1;
                max += 1;
            },
            ')' => {
                min -= 1;
                max -= 1;
            },
            _ => (),
        }
        if max < 0 {
            return false;
        }
        min = min.max(0);
    }
    min == 0
}

pub fn main() {
    let str = String::from("()");
    assert!(check_valid_string(str));
    let str = String::from("(*)");
    assert!(check_valid_string(str));
    let str = String::from("(*))");
    assert!(check_valid_string(str));
    let str = String::from("(((((()*)(*)*))())())(()())())))((**)))))(()())()");
    assert!(!check_valid_string(str));
    let str = String::from("((*)");
    assert!(check_valid_string(str));
    let str = String::from("((((()(()()()*()(((((*)()*(**(())))))(())()())(((())())())))))))(((((())*)))()))(()((*()*(*)))(*)()");
    assert!(check_valid_string(str));
    print_pass(NAME)
}
