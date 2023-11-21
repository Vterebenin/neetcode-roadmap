use crate::utils::print_pass;

const NAME: &str = "missing-number";


pub fn missing_number(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .enumerate()
        .fold(0, |xor, (i, x)| xor ^ i as i32 + 1 ^ x)}

pub fn main() {
    let x: Vec<i32> = vec![0, 1, 2, 4, 5];
    assert_eq!(missing_number(x), 3);
    print_pass(NAME)
}
