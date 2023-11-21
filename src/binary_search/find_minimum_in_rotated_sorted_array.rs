use crate::utils::print_pass;
use std::cmp::Ordering;

const NAME: &str = "find-minimum-in-rotated-sorted-array";


pub fn find_min(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 { return *nums.first().unwrap(); }

    let mut min = nums.first().unwrap();
    let (mut low, mut high) = (0, nums.len());
    while low < high {
        let mid = low + (high - low) / 2;
        match nums[mid].cmp(&min) {
            Ordering::Less => {
                min = &nums[mid];
                high = mid;
            },
            Ordering::Greater => {
                low = mid + 1;
            },
            Ordering::Equal => (),
        }
    }
    *min
}

pub fn main() {
    let nums: Vec<i32> = vec![3, 4, 5, 1, 2];
    assert_eq!(find_min(nums), 1);
    print_pass(NAME)
}
