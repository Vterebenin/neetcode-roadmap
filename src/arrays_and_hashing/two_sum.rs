
use std::collections::HashMap;

use crate::utils::print_pass;

const NAME: &str = "two_sum";
const LINK: &str = "https://leetcode.com/problems/two-sum/";
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (i, n) in nums.iter().enumerate() {
        if map.contains_key(&(target - n)) {
            return vec![map[&(target - n)], i as i32];
        }
        map.insert(*n, i as i32);
    }
    vec![]
}

pub fn main() {
    let items = vec![1, 3, 2, 3];
    let target = 4;
    assert_eq!(two_sum(items, target), vec![0, 1]);
    let items = vec![1, 3, 2, 3];
    let target = 7;
    assert_eq!(two_sum(items, target), vec![]);
    print_pass(NAME, LINK)
}
