use crate::utils::print_pass;

const NAME: &str = "maximum-subarray";
const LINK: &str = "https://leetcode.com/problems/maximum-subarray/";

pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut max_val: i32 = 0;

    for (idx, val) in nums.iter().enumerate() {
        if idx as i32 > max_val {
            return false;
        }
        max_val = max_val.max((idx as i32 + *val as i32) as i32);
    }
    return true
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
