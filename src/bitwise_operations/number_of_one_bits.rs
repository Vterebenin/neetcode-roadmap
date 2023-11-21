use crate::utils::print_pass;

const NAME: &str = "number-of-1-bits";


pub fn hamming_weight(n: u32) -> i32 {
    n.count_ones() as i32
}

pub fn main() {
    let x: u32 = 43261596;
    assert_eq!(hamming_weight(x), 12);
    print_pass(NAME)
}
