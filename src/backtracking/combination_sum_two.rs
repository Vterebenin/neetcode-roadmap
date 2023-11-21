use crate::utils::print_pass;

const NAME: &str = "combination-sum-ii";


pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort_unstable();
    let mut rez = vec![];
    dfs(&candidates, &mut vec![], &mut rez, target);
    rez
}

fn dfs(candidates: &[i32], cur: &mut Vec<i32>, dp: &mut Vec<Vec<i32>>, target: i32) {
    if target == 0 {
        dp.push(cur.clone());
        return;
    }

    for i in 0..candidates.len() {
        if candidates[i] > target {
            break;
        }
        if i != 0 && candidates[i] == candidates[i - 1] {
            continue;
        }
        cur.push(candidates[i]);
        dfs(&candidates[i + 1..], cur, dp, target - candidates[i]);
        cur.pop();
    }
}

pub fn main() {
    let nums: Vec<i32> = vec![10,1,2,7,6,1,5];
    let target = 8;
    let answer = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1,7], vec![2,6]];
    assert_eq!(combination_sum2(nums, target), answer);
    let nums = vec![4,4,2,1,4,2,2,1,3];
    let target = 6;
    let answer = vec![vec![1,1,2,2],vec![1,1,4],vec![1,2,3],vec![2,2,2],vec![2,4]];
    assert_eq!(combination_sum2(nums, target), answer);
    print_pass(NAME);
}

