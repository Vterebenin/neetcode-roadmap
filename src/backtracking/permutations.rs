use crate::utils::print_pass;

const NAME: &str = "permutations";
const LINK: &str = "https://leetcode.com/problems/permutations/";


pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    pub fn backtrack(
        res: &mut Vec<Vec<i32>>, 
        nums: &mut Vec<i32>, 
        permutation: &mut Vec<i32>, 
        used: &mut Vec<bool>,
    ) {
        if permutation.len() == nums.len() {
           res.push(permutation.to_vec()); 
           return;
        }
        for i in 0..nums.len() {
            if used[i] { 
                continue;
            }
            used[i] = true;
            if !permutation.contains(&nums[i]) {
                permutation.push(nums[i]);
            }
            backtrack(res, nums, permutation, used);
            used[i] = false;
            permutation.pop();
        }
    }
    let mut res: Vec<Vec<i32>> = vec![];
    let mut used = vec![false; nums.len()];
    let mut permutation: Vec<i32> = vec![];
    backtrack(&mut res, &mut nums, &mut permutation, &mut used);
    res
}

pub fn main() {
    let nums: Vec<i32> = vec![1,2,3];
    let answer = vec![
        vec![1,2,3],
        vec![1,3,2],
        vec![2,1,3],
        vec![2,3,1],
        vec![3,1,2],
        vec![3,2,1]
    ];
    assert_eq!(subsets(nums), answer);
    print_pass(NAME, LINK)
}

