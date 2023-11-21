use crate::utils::print_pass;

const NAME: &str = "longest-increasing-subsequence";


pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 { return nums.len() as i32; }

    let mut dp = vec![1; nums.len()];
    let mut max_lis = 1;

    for i in 1..nums.len() {
        let mut tmp_max = 0;
        for j in 0..i {
            if nums[i] > nums[j] {
                tmp_max = tmp_max.max(dp[j]);
            }
            dp[i] = tmp_max + 1;
        }
        max_lis = max_lis.max(dp[i]);
    }
    max_lis
}
pub fn main() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    assert_eq!(length_of_lis(nums), 4);
    let nums = vec![0,1,0,3,2,3];
    assert_eq!(length_of_lis(nums), 4);
    print_pass(NAME)
}
