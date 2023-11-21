use crate::utils::print_pass;
use std::{cmp::Ordering, vec};

const NAME: &str = "search-a-2d-matrix";


pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (mut low, mut high) = (0, matrix.len());
    let mut target_index: i32 = -1;
    while low < high {
        let mid = low + (high - low) / 2;
        let mid_arr = matrix[mid].clone();
        if mid_arr.first().unwrap() > &target {
            high = mid;
        } else if mid_arr.last().unwrap() < &target {
            low = mid + 1;
        } else {
            target_index = mid as i32;
            break;
        }
    }
    if target_index == -1 { return false };
    let nums = matrix[target_index as usize].clone();
    
    let (mut low, mut high) = (0, nums.len());
    while low < high {
        let mid = low + (high - low) / 2;
        match nums[mid].cmp(&target) {
            Ordering::Equal => return true,
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
        }
    }
    false
}

pub fn main() {
    let nums: Vec<Vec<i32>> = vec![vec![1,3,5,7],vec![10,11,16,20], vec![23,30,34,60]];
    let target: i32 = 3;
    assert_eq!(search_matrix(nums, target), true);
    print_pass(NAME)
}
