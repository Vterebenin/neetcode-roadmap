use std::collections::HashMap;

use crate::utils::print_pass;

const NAME: &str = "partition-labels";
const LINK: &str = "https://leetcode.com/problems/partition-labels/";

pub fn partition_labels(s: String) -> Vec<i32> {
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut answer = vec![];
    for (index, char) in s.chars().enumerate() {
        map
            .entry(char)
            .and_modify(|c| *c = index)
            .or_insert(index);
    }
    let mut start = 0;
    let mut end = 0;
    for (index, char) in s.chars().enumerate() {
        let val = *map.get(&char).unwrap();
        end = end.max(val);
        if index == end {
            answer.push((end - start + 1) as i32);
            start = end + 1;
        }
        
    }
    answer
}

pub fn main() {
    let s = "ababcbacadefegdehijhklij".to_string();
    assert_eq!(partition_labels(s), vec![9, 7, 8]);
    print_pass(NAME, LINK)
}
