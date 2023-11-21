use crate::utils::print_pass;

const NAME: &str = "counting-bits";


pub fn count_bits(n: i32) -> Vec<i32> {
    let n = (n + 1) as usize;
    let mut count = vec![0; n];
    for i in 1..n {
        count[i] = count[i & (i - 1)] + 1;
    }
    count
}

pub fn main() {
    let n = 5;
    assert_eq!(count_bits(n), vec![0,1,1,2,1,2]);
    print_pass(NAME)
}
