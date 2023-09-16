use crate::utils::print_pass;

const NAME: &str = "climbing-stairs";
const LINK: &str = "https://leetcode.com/problems/climbing-stairs/";

pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.is_empty() { return 0 }
    
    let mut prev1 = 0;
    let mut prev2 = 0;
    for n in nums.iter() {
        let tmp = prev1;
        prev1 = (prev2 + n).max(prev1);
        prev2 = tmp;
    }
    prev1
}

pub fn main() {
    let nums = vec![1,2,3,1];
    // 1 + 2
    assert_eq!(rob(nums), 4);
    let nums = vec![2,1,1,2];
    // 1 + 2
    assert_eq!(rob(nums), 4);
    print_pass(NAME, LINK)
}
