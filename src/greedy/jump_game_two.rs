use crate::utils::print_pass;

const NAME: &str = "jump-game-ii";
const LINK: &str = "https://leetcode.com/problems/jump-game-ii/";

pub fn jump(nums: Vec<i32>) -> i32 {
    let (mut furr, mut end, mut res, l1) = (0,0,0, nums.len());
    for i in 0..l1 - 1 {   
        furr = furr.max(i + nums[i] as usize);        
        if i == end { 
            res += 1;
            end = furr
        }
    }
    res as i32
}

pub fn main() {
    let nums = vec![2,3,1,1,4];
    assert_eq!(jump(nums), 2);
    let nums = vec![5,3,1,1,4,3,4,3,2,1];
    assert_eq!(jump(nums), 3);
    let nums = vec![2,3,0,1,4];
    assert_eq!(jump(nums), 2);
    let nums = vec![1,2];
    assert_eq!(jump(nums), 1);
    let nums = vec![1,2,0,1];
    assert_eq!(jump(nums), 2);
    let nums = vec![1, 2, 3];
    assert_eq!(jump(nums), 2);
    let nums = vec![2, 3, 1];
    assert_eq!(jump(nums), 1);
    let nums = vec![3, 4, 3, 2, 5, 4, 3];
    assert_eq!(jump(nums), 3);
    let nums = vec![5,9,3,2,1,0,2,3,3,1,0,0];
    assert_eq!(jump(nums), 3);
    print_pass(NAME, LINK)
}
