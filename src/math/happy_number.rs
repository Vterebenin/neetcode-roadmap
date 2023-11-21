use crate::utils::print_pass;

const NAME: &str = "happy-number";


pub fn is_happy(n: i32) -> bool {
    if n == 1 { return true; }
    let mut number = n;
    let mut previous = vec![];
    while number != 1 && !previous.contains(&number) {
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
        if number == 1 {
            return true;
        }
    }
    false
}

pub fn main() {
    assert_eq!(is_happy(19), true);
    print_pass(NAME)
}
