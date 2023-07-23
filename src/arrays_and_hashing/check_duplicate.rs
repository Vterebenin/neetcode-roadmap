use std::collections::HashSet;
use std::iter::FromIterator;

use crate::utils::print_pass;

const NAME: &str = "contains_duplicate";
const LINK: &str = "https://leetcode.com/problems/contains-duplicate/";
fn contains_duplicate(nums: Vec<i32>) -> bool {
    nums.len() != HashSet::<i32>::from_iter(nums).len()
}

pub fn main() {
    let items = vec![1, 3, 2, 3];
    assert!(contains_duplicate(items));
    let items = vec![1, 3, 2, 4];
    assert!(!contains_duplicate(items));
    print_pass(NAME, LINK)
}
