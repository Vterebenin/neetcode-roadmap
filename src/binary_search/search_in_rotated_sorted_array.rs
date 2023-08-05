use crate::utils::print_pass;

const NAME: &str = "search-in-rotated-sorted-array";
const LINK: &str = "https://leetcode.com/problems/search-in-rotated-sorted-array/";

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target { return mid as i32 };
        if nums[mid] >= nums[left] { 
            if target > nums[mid] || target < nums[left] {
                left = mid + 1;
            }
            else {
                right = mid - 1;
            }
        } else {
            if target < nums[mid] || target > nums[right] {
                right = mid - 1;
            } else { 
                left = mid + 1;
            }
        };
    }

    -1
}

pub fn main() {
    let nums: Vec<i32> = vec![3, 4, 5, 1, 2];
    assert_eq!(search(nums, 3), 0);
    let nums: Vec<i32> = vec![3, 4, 5, 7, 8, 1, 2];
    assert_eq!(search(nums, 8), 4);
    let nums: Vec<i32> = vec![3, 4, 5, 7, 8, 1, 2];
    assert_eq!(search(nums, 99), -1);
    print_pass(NAME, LINK)
}
