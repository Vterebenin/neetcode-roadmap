use crate::utils::print_pass;
use std::cmp;

const NAME: &str = "container-with-most-water";
const LINK: &str = "https://leetcode.com/problems/container-with-most-water/";

fn max_area(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut prefix = 0;
    let mut postfix = height.len() - 1;
    while prefix < postfix {
        let area_height: i32 = cmp::min(height[prefix], height[postfix]);
        let area_length: usize = postfix - prefix;
        max = cmp::max(max, area_length * area_height as usize);
        if height[prefix] < height[postfix] {
            prefix += 1;
        } else {
            postfix -= 1;
        }
    }
    max as i32
}

pub fn main() {
    let height = vec![1,8,6,2,5,4,8,3,7];
    assert_eq!(max_area(height), 49);
    let height = vec![1, 1];
    assert_eq!(max_area(height), 1);
    print_pass(NAME, LINK)
}
