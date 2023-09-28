use crate::utils::print_pass;

const NAME: &str = "decode-ways";
const LINK: &str = "https://leetcode.com/problems/decode-ways/";

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    // Amount is guaranteed to be <= 10^4, so use 10^5 as sentinel
    const BIG_NUM: i32 = 100000;
    let amount = amount as usize;
    // Fill DP array with the sentinel to flag which paths are impossible
    let mut dp = vec![BIG_NUM; amount + 1];
    // The initial available path is 0 coins for 0 amount
    dp[0] = 0;
    for i in 1..=amount {
        // Use available change to jump from previous amounts
        for coin in coins.iter().map(|c| *c as usize) {
            if coin <= i {
                dp[i] = dp[i].min(dp[i-coin] + 1);
            }
        }
    }
    match dp[amount] {
        change if change < BIG_NUM => change,
        _ => -1,
    }
}

pub fn main() {
    let coins = vec![1, 2, 5];
    let amount = 11;
    assert_eq!(coin_change(coins, amount), 3);
    let coins = vec![2];
    let amount = 3;
    assert_eq!(coin_change(coins, amount), -1);
    let coins = vec![1];
    let amount = 0;
    assert_eq!(coin_change(coins, amount), 0);
    print_pass(NAME, LINK)
}
