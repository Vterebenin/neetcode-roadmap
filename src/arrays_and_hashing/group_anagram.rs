use crate::utils::print_pass;
use std::collections::HashMap;

const NAME: &str = "group-anagrams";
const LINK: &str = "https://leetcode.com/problems/group-anagrams/";

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut h = HashMap::new();

    for s in strs {
        let mut key: Vec<char> = s.chars().collect();
        key.sort();
        h.entry(key).or_insert(vec![]).push(s);
    }

    h.values().cloned().collect()
}

pub fn main() {
    let strings = vec![
        String::from("eat"),
        String::from("tea"),
        String::from("tan"),
        String::from("ate"),
        String::from("nat"),
        String::from("bat")
    ];
    assert!(group_anagrams(strings).len() == 3);
    print_pass(NAME, LINK);
}
