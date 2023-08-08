use crate::utils::print_pass;

const NAME: &str = "permutation-in-string";
const LINK: &str = "https://leetcode.com/problems/permutation-in-string/";

pub fn check_inclusion(s1: String, s2: String) -> i32 {
    let mut max_len: usize = 0;
    let mut pos: [usize;128] = [0;128];
    let mut start: usize = 0;

    for (end, ch) in s.chars().enumerate() {
        start = start.max(pos[ch as usize]);

        max_len = max_len.max(end-start+1);

        pos[ch as usize] = end + 1;
    }

    max_len as i32
}

pub fn main() {
    let s1: String = String::from("ab");
    let s2: String = String::from("eidbaooo");
    assert_eq!(check_inclusion(s1, s2), true);
    print_pass(NAME, LINK)
}
