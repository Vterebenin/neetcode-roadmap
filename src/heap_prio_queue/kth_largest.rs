use crate::utils::print_pass;
use std::collections::BinaryHeap;

const NAME: &str = "k-closest-points-to-origin";



// in rust you could just select nth unstable
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut k = k as usize;
    let mut pq = BinaryHeap::from(nums);
    let mut response = 0;
    while k > 0 {
        if let Some(p) = pq.pop() {
            response = p;
        }
        k -= 1;
    }
    // let n = nums.len();
    // *nums.select_nth_unstable(n - k as usize).1
    response
}

pub fn main() {
    let nums = vec![3,2,1,5,6,4];
    let k = 2;
    assert_eq!(find_kth_largest(nums, k), 5);
    print_pass(NAME);
}
