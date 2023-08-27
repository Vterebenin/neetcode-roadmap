use crate::utils::print_pass;

const NAME: &str = "subsets";
const LINK: &str = "https://leetcode.com/problems/subsets/";


pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    pub fn backtrack(
        res: &mut Vec<Vec<i32>>, 
        temp: &mut Vec<i32>, 
        nums: &mut Vec<i32>, 
        start: usize,
    ) {
        res.push(temp.to_vec());
        for i in start..nums.len() {
            temp.push(nums[i]);
            backtrack(res, temp, nums, i + 1);
            temp.pop();
        }
    }
    let mut res: Vec<Vec<i32>> = vec![];
    let mut temp = vec![];
    let start = 0;
    backtrack(&mut res, &mut temp, &mut nums, start);
    res
}

pub fn main() {
    let nums: Vec<i32> = vec![1,2,3];
    let answer = vec![
        vec![], 
        vec![1], 
        vec![1, 2], 
        vec![1, 2, 3], 
        vec![1, 3], 
        vec![2], 
        vec![2, 3], 
        vec![3]
    ];
    assert_eq!(subsets(nums), answer);
    print_pass(NAME, LINK)
}
