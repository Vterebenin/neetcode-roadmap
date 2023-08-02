use crate::utils::print_pass;
use std::cmp::Ordering;

const NAME: &str = "binary-search";
const LINK: &str = "https://leetcode.com/problems/binary-search/";

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut low, mut high) = (0, nums.len());
    while low < high {
        let mid = low + (high - low) / 2;
        match nums[mid].cmp(&target) {
            Ordering::Equal => return mid as i32,
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
        }
    }
    -1
}

pub fn main() {
    let nums: Vec<i32> = vec![-1,0,3,5,9,12];
    let target: i32 = 9;
    assert_eq!(search(nums, target), 4);
    print_pass(NAME, LINK)
}
