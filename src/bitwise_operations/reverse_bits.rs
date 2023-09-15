use crate::utils::print_pass;

const NAME: &str = "binary-search";
const LINK: &str = "https://leetcode.com/problems/binary-search/";

pub fn reverse_bits(mut x: u32) -> u32 {
    let mut result = 0;
    let mut bit_position = 31;

    while x != 0 {
        println!("{}", format!("{:b}", x));
        result |= (x & 1) << bit_position;
        x >>= 1;
        bit_position -= 1;
    }
    result
}

pub fn main() {
    let x: u32 = 43261596;
    assert_eq!(reverse_bits(x), 964176192);
    print_pass(NAME, LINK)
}
