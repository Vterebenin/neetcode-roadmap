use crate::utils::print_pass;

const NAME: &str = "longest-substring-without-repeating-characters";
const LINK: &str = "https://leetcode.com/problems/longest-substring-without-repeating-characters/";

pub fn length_of_longest_substring(s: String) -> i32 {
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
    let item: String = String::from("abcabcbb");
    assert_eq!(length_of_longest_substring(item), 3);
    let item: String = String::from("au");
    assert_eq!(length_of_longest_substring(item), 2);
    let item: String = String::from("dvdf");
    assert_eq!(length_of_longest_substring(item), 3);
    print_pass(NAME, LINK)
}
