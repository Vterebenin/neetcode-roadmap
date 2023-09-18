use crate::utils::print_pass;

const NAME: &str = "min-cost-climbing-stairs";
const LINK: &str = "https://leetcode.com/problems/min-cost-climbing-stairs/";

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut costs_copy = cost.clone();
    costs_copy.push(0);
    for i in (0..costs_copy.len() - 3).rev() {
        costs_copy[i] += costs_copy[i + 1].min(costs_copy[i + 2])
    }
    costs_copy[0].min(costs_copy[1])
}

pub fn main() {
    let costs = vec![10, 15, 20];
    assert_eq!(min_cost_climbing_stairs(costs), 15);
    let costs = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    assert_eq!(min_cost_climbing_stairs(costs), 6);
    print_pass(NAME, LINK)
}
