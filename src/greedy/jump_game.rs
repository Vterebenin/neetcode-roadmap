use crate::utils::print_pass;

const NAME: &str = "maximum-subarray";
const LINK: &str = "https://leetcode.com/problems/maximum-subarray/";

pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut n = 1;
    for i in (0..nums.len() - 1).rev() {
        n = if nums[i] < n { n + 1 } else { 1 };
    }
    n == 1
}

pub fn main() {
    let nums: Vec<i32> = vec![3,2,1,0,4];
    assert_eq!(can_jump(nums), false);
    let nums: Vec<i32> = vec![2,3,1,1,4];
    assert_eq!(can_jump(nums), true);
    let nums: Vec<i32> = vec![0, 1];
    assert_eq!(can_jump(nums), false);
    let nums: Vec<i32> = vec![1, 0, 2];
    assert_eq!(can_jump(nums), false);
    let nums: Vec<i32> = vec![2, 0, 0];
    assert_eq!(can_jump(nums), true);
    let nums: Vec<i32> = vec![4, 3, 2, 1];
    assert_eq!(can_jump(nums), true);
    let nums: Vec<i32> = vec![2,1,2,2,1,2,2,2];
    assert_eq!(can_jump(nums), true);
    let nums: Vec<i32> = vec![3,4,0,1,0,0,3,0];
    assert_eq!(can_jump(nums), false);
    print_pass(NAME, LINK)
}
