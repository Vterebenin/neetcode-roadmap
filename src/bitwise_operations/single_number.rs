use crate::utils::print_pass;

const NAME: &str = "single-number";


pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, x| acc ^ x)
}

pub fn main() {
    let x: Vec<i32> = vec![4, 1, 2, 1, 2];
    assert_eq!(single_number(x), 4);
    print_pass(NAME)
}
