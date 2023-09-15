use crate::utils::print_pass;

const NAME: &str = "climbing-stairs";
const LINK: &str = "https://leetcode.com/problems/climbing-stairs/";

pub fn climb_stairs(n: i32) -> i32 {
    (0..n)
        .fold((1, 0), |(res, prev), _| (res + prev, res))
        .0
}

pub fn main() {
    let num = 3;
    // 1 + 2
    assert_eq!(climb_stairs(num), 3);
    let num = 4;
    // 3 + 2
    assert_eq!(climb_stairs(num), 5);
    let num = 5;
    // 3 + 5
    assert_eq!(climb_stairs(num), 8);
    print_pass(NAME, LINK)
}
