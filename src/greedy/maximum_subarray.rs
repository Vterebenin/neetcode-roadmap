use crate::utils::print_pass;

const NAME: &str = "maximum-subarray";
const LINK: &str = "https://leetcode.com/problems/maximum-subarray/";

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut table = vec![0; nums.len()];
    table[0] = nums[0];
    let mut max = table[0];
    for i in 1..nums.len() {
        table[i] = nums[i] + 0.max(table[i - 1]);
        max = max.max(table[i]);
    }
    max
}

pub fn main() {
    let temps: Vec<i32> = vec![-2,1,-3,4,-1,2,1,-5,4];
    assert_eq!(max_sub_array(temps), 6);
    print_pass(NAME, LINK)
}
