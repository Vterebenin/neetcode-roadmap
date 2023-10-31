use crate::utils::print_pass;

const NAME: &str = "edit-distance";
const LINK: &str = "https://leetcode.com/problems/edit-distance/";

pub fn min_distance(word1: String, word2: String) -> i32 {
    let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];
    for j in 1..=word2.len() {
        dp[0][j] = j;
    }
    for i in 1..=word1.len() {
        dp[i][0] = i;
    }
    for i in 1..=word1.len() {
        for j in 1..=word2.len() {
            let v = if word1.as_bytes()[i - 1] as char != word2.as_bytes()[j - 1] as char {
                1
            } else {
                0
            };
            let last_item = dp[i - 1][j - 1] + v;
            let items = vec![dp[i - 1][j] + 1, dp[i][j - 1] + 1, last_item];
            dp[i][j] = *items.iter().min().unwrap();
        }
    }
    dp[word1.len()][word2.len()] as i32
}

pub fn main() {
    let word1 = String::from("horse");
    let word2 = String::from("ros");
    assert_eq!(min_distance(word1, word2), 3);
    print_pass(NAME, LINK)
}
