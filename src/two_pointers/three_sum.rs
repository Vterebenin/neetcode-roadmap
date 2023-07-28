use crate::utils::print_pass;

const NAME: &str = "three-sum";
const LINK: &str = "https://leetcode.com/problems/3sum/";

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    nums.sort();

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            match sum {
                x if x > 0 => right -= 1,
                x if x < 0 => left += 1,
                _ => {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;

                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
            }
        }
    }

    res
}

pub fn main() {
    let nums = vec![-1,0,1,2,-1,-4];
    let answer = vec![vec![-1,-1,2], vec![-1,0,1]];
    assert_eq!(three_sum(nums), answer);
    print_pass(NAME, LINK)
}
