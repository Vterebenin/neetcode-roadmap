use crate::utils::print_pass;

const NAME: &str = "reverse-bits";


pub fn reverse_bits(x: u32) -> u32 {
    x.reverse_bits()
}

pub fn main() {
    let x: u32 = 43261596;
    assert_eq!(reverse_bits(x), 964176192);
    print_pass(NAME)
}
