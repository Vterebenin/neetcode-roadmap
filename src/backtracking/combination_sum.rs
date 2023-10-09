use crate::utils::print_pass;

const NAME: &str = "combination-sum";
const LINK: &str = "https://leetcode.com/problems/combination-sum/";


fn combi(answer: &mut Vec<Vec<i32>>, buffer: &mut Vec<i32>, candidates: &[i32], target: &i32) {
    buffer.push(candidates[0]);

    if buffer.iter().sum::<i32>() == *target {
        answer.push(buffer.clone());
    } else if buffer.iter().sum::<i32>()<*target {
        for i in 0..candidates.len() {
            combi(answer, buffer, &candidates[i..], target);
        }
    }

    buffer.pop();
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut answer: Vec<Vec<i32>> = vec![];
    let mut buffer: Vec<i32>;
    for i in 0..candidates.len() {
        buffer = vec![];
        combi(&mut answer, &mut buffer, &candidates[i..], &target);
    }
    answer
}

pub fn main() {
    let nums: Vec<i32> = vec![2, 3, 6, 7];
    let target = 7;
    let answer = vec![vec![2, 2, 3], vec![7]];
    assert_eq!(combination_sum(nums, target), answer);
    print_pass(NAME, LINK)
}

