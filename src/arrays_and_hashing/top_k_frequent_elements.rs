use crate::utils::print_pass;
use std::collections::HashMap;

const NAME: &str = "top-k-frequent-elements";


pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut frequency: HashMap<i32, i32> = HashMap::new();
    for num in nums.iter() {
        frequency.entry(num.to_owned()).and_modify(|counter| *counter += 1).or_insert(1);
    }
    let mut counter_vec: Vec<_> = frequency.into_iter().collect();
    counter_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let counter: Vec<i32> = counter_vec.iter().map(|item| item.0).collect::<Vec<i32>>();
    counter[0..k as usize].to_vec()
}

pub fn main() {
    let items = vec![1, 1, 1, 2, 2, 3];
    assert_eq!(top_k_frequent(items, 2), vec![1, 2]);
    print_pass(NAME);
}
