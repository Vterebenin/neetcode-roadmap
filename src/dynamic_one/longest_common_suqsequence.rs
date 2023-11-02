use crate::utils::print_pass;

const NAME: &str = "longest-common-subsequence";
const LINK: &str = "https://leetcode.com/problems/longest-common-subsequence/";


pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let text1_bytes = text1.as_bytes();
    let text2_bytes = text2.as_bytes();
    let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];

    for i in 1..=text1.len() {
        for j in 1..=text2.len() {
            if text1_bytes[i - 1] == text2_bytes[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[text1.len()][text2.len()] as i32
}

pub fn main() {
    let text1 = String::from("abcde");
    let text2 = String::from("ace");
    assert_eq!(longest_common_subsequence(text1, text2), 3);
    let text1 = String::from("abc");
    let text2 = String::from("abc");
    assert_eq!(longest_common_subsequence(text1, text2), 3);
    let text1 = String::from("abc");
    let text2 = String::from("def");
    assert_eq!(longest_common_subsequence(text1, text2), 0);
    let text1 = String::from("aaa");
    let text2 = String::from("aaaa");
    assert_eq!(longest_common_subsequence(text1, text2), 3);
    print_pass(NAME, LINK)
}
