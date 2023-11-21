use crate::utils::print_pass;

const NAME: &str = "sum-of-two-integers";

pub fn get_sum(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    let sum = a ^ b;
    let carry = (a & b) << 1;
    get_sum(sum, carry)
}

pub fn main() {
    assert_eq!(get_sum(1, 2), 3);
    assert_eq!(get_sum(-1, 1), 0);
    print_pass(NAME)
}
