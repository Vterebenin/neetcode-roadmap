use crate::utils::print_pass;

const NAME: &str = "subsets-ii";



pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    pub fn backtrack(
        res: &mut Vec<Vec<i32>>, 
        temp: &mut Vec<i32>, 
        nums: &mut Vec<i32>, 
        start: usize,
    ) {
        if res.iter().all(|arr| { arr != temp }) {
            res.push(temp.to_vec());
        }

        for i in start..nums.len() {
            temp.push(nums[i]);
            backtrack(res, temp, nums, i + 1);
            temp.pop();
        }
    }
    nums.sort();
    let mut res: Vec<Vec<i32>> = vec![];
    let mut temp = vec![];
    let start = 0;
    backtrack(&mut res, &mut temp, &mut nums, start);
    res
}

pub fn main() {
    let nums: Vec<i32> = vec![1,2,2];
    let answer = vec![
        vec![],
        vec![1],
        vec![1,2],
        vec![1,2,2],
        vec![2],
        vec![2,2],
    ];
    assert_eq!(subsets(nums), answer);
    print_pass(NAME)
}

