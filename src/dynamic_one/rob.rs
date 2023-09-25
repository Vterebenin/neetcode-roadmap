use crate::utils::print_pass;

const NAME: &str = "house-robber-ii";
const LINK: &str = "https://leetcode.com/problems/min-cost-climbing-stairs/";

pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 { return nums[0] };
    let mut rob_odd = vec![0; nums.len()];
    let mut rob_even = vec![0; nums.len()];
    rob_odd[0] = nums[0];
    rob_odd[1] = nums[0].max(nums[1]);
    let mut reversed_nums: Vec<i32> = nums.clone();
    reversed_nums.reverse();
    rob_even[0] = reversed_nums[0];
    rob_even[1] = reversed_nums[0].max(reversed_nums[1]);
    for i in 2..nums.len() - 1 {
        let prev_index = i - 2;
        let new_rob_sum = nums[i] + rob_odd[prev_index];
        rob_odd[i] = new_rob_sum.max(rob_odd[i - 1]);
    }
    for i in 2..reversed_nums.len() - 1 {
        let prev_index = i - 2;
        let new_rob_sum = reversed_nums[i] + rob_even[prev_index];
        rob_even[i] = new_rob_sum.max(rob_even[i - 1]);
    }
    rob_odd[nums.len() - 2].max(rob_even[nums.len() - 2])
}

pub fn main() {
    let costs = vec![1, 2, 3, 1];
    assert_eq!(rob(costs), 4);
    let costs = vec![2, 3, 2];
    assert_eq!(rob(costs), 3);
    let costs = vec![1, 2, 1, 1];
    assert_eq!(rob(costs), 3);
    let costs = vec![0, 0];
    assert_eq!(rob(costs), 0);
    print_pass(NAME, LINK)
}
