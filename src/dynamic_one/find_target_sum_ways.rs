use std::collections::HashMap;

use crate::utils::print_pass;

const NAME: &str = "target-sum";
const LINK: &str = "https://leetcode.com/problems/target-sum/";

pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut dp: HashMap<(i32, i32), i32> = HashMap::new();
    pub fn backtrack(i: i32, total: i32, nums: &Vec<i32>, target: i32, dp: &mut HashMap<(i32, i32), i32>) -> i32 {
        let curr = i as usize;
        if curr == nums.len() {
            if total == target {
                return 1;
            } else {
                return 0;
            }
        } 
        if dp.contains_key(&(i, total)) {
            return *dp.get(&(i, total)).unwrap();
        }
        let result = backtrack(i + 1, total + nums[curr], nums, target, dp) + backtrack(i + 1, total - nums[curr], nums, target, dp);
        dp
            .insert(
                (i, total),
                result
            );
        *dp.get(&(i, total)).unwrap()
    }
    backtrack(0, 0, &nums, target, &mut dp)
}
pub fn can_sum(nums: Vec<i32>, target: i32) -> bool {
    let target = target as usize;
    let mut table = vec![false; target + 1];
    table[0] = true;

    for i in 0..=target {
        if table[i] {
            for num in nums.iter() {
                let num = *num as usize;
                if i + num <= target {
                    table[i + num] = true;
                }
            }
        }
    }
    table[target]
}

pub fn main() {
    let nums = vec![1, 1, 1, 1, 1];
    let target = 3;
    assert_eq!(find_target_sum_ways(nums, target), 5);
    let nums = vec![1, 1, 1, 1, 1];
    let target = 9;
    assert_eq!(can_sum(nums, target), true);
    print_pass(NAME, LINK)
}
