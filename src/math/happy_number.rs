use crate::utils::print_pass;

const NAME: &str = "happy-number";
const LINK: &str = "https://leetcode.com/problems/happy-number/";

pub fn is_happy(n: i32) -> bool {
    let mut number = n;
    let mut previous = vec![];
    while number != 1 && !previous.contains(&number) {
        if number == 1 {
            return true;
        }
        previous.push(number);
        number = number.to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>()
            .into_iter()
            .fold(0, |mut acc, e| {
                acc += e*e;
                acc
            });
    }
    false
}

pub fn main() {
    assert_eq!(is_happy(19), true);
    print_pass(NAME, LINK)
}
