use crate::utils::print_pass;

const NAME: &str = "product-of-array-except-self";
const LINK: &str = "https://leetcode.com/problems/product-of-array-except-self/";

// basically multiply from left to right and then from right to left, we could do this in O(n)
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![1; nums.len()];
    let mut prefix = 1;
    let mut postfix = 1;

    let mut postfix_index = nums.len() - 1;
    
    for prefix_index in 0..nums.len() {
        res[prefix_index] *= prefix;
        res[postfix_index] *= postfix;

        prefix *= nums[prefix_index];
        postfix *= nums[postfix_index];
        if postfix_index == 0 {
            break;
        }
        postfix_index -= 1;
    }

    res
}

pub fn main() {
    let items = vec![1, 2, 3, 4];
    assert_eq!(product_except_self(items), vec![24, 12, 8, 6]);
    print_pass(NAME, LINK);
}
