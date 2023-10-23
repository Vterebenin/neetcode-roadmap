use crate::utils::print_pass;

const NAME: &str = "palindromic-substrings";
const LINK: &str = "https://leetcode.com/problems/palindromic-substrings/";

pub fn count_substrings(s: String) -> i32 {
    let s = s.into_bytes();
    let count = |i, j| {
        ((0..=i).rev())
            .zip(j..s.len())
            .try_fold(0, |a, (i, j)| if s[i] == s[j] { Ok(a + 1) } else { Err(a) })
            .unwrap_or_else(|a| a)
    };
    (0..s.len()).map(|i| count(i, i) + count(i, i + 1)).sum()
}

pub fn main() {
    let string = String::from("abc");
    assert_eq!(count_substrings(string), 3);
    let string = String::from("aaa");
    assert_eq!(count_substrings(string), 6);
    print_pass(NAME, LINK)
}
