use crate::utils::print_pass;

const NAME: &str = "coin-change-ii";
const LINK: &str = "https://leetcode.com/problems/coin-change-ii/";

pub fn change(target: i32, nums: Vec<i32>) -> i32 {
    let mut dp = vec![0; target as usize + 1];
    dp[0] = 1;
    for num in &nums {
        let num = *num as usize;
        for i in num..dp.len() {
            dp[i] += dp[i - num];
        }
    }
    dp[target as usize]
}

pub fn main() {
    let coins = vec![1, 2, 5];
    let amount = 5;
    assert_eq!(change(amount, coins), 4);
    let coins = vec![2];
    let amount = 3;
    assert_eq!(change(amount, coins), 0);
    let coins = vec![10];
    let amount = 10;
    assert_eq!(change(amount, coins), 1);
    print_pass(NAME, LINK)
}

