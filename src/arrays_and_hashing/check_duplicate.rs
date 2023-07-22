use colored::Colorize;
use std::collections::HashSet;
use std::iter::FromIterator;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    nums.len() != HashSet::<i32>::from_iter(nums).len()
}

pub fn main() {
    let items = vec![1, 3, 2, 3];
    assert!(contains_duplicate(items));
    let items = vec![1, 3, 2, 4];
    assert!(!contains_duplicate(items));
    println!("{}", "contains_duplicate passed!".green());
}
