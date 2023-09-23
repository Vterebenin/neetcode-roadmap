use crate::utils::print_pass;

const NAME: &str = "longest-consecutive-sequence";
const LINK: &str = "https://leetcode.com/problems/longest-consecutive-sequence/";

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: std::collections::HashSet<_> = nums.iter().collect();
    set.iter()
        .filter(|&&x| !set.contains(&(x - 1))) 
        .map(|&&x| (x..).take_while(|x| set.contains(x)).count())
        .max()
        .unwrap_or(0) as i32
}

pub fn main() {
    let nums = vec![0,3,7,2,5,8,4,6,0,1];
    assert_eq!(longest_consecutive(nums), 9);
    print_pass(NAME, LINK);
}
