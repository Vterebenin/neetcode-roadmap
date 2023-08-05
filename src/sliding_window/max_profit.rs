use crate::utils::print_pass;
use std::cmp::max;

const NAME: &str = "best-time-to-buy-and-sell-stock";
const LINK: &str = "https://leetcode.com/problems/best-time-to-buy-and-sell-stock/";

pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 { return 0 };
    let mut max_profit = 0;
    let mut cheapest = prices[0];
    for index in 1..prices.len() {
        if prices[index] < cheapest {
            cheapest = prices[index];
            continue;
        }
        let profit = prices[index] - cheapest;
        if profit > max_profit {
            max_profit = profit;
        }
    }
    max(0, max_profit)
}

pub fn main() {
    let prices: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(max_profit(prices), 5);
    print_pass(NAME, LINK)
}
