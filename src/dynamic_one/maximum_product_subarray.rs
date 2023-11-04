use crate::utils::print_pass;

const NAME: &str = "longest-increasing-subsequence";
const LINK: &str = "https://leetcode.com/problems/longest-increasing-subsequence/";

// Let product[i]=nums[0]∗nums[1]∗...∗nums[i].
// If product[i-1] > 0, and nums[i]<0, then product[i]<0.
// If product[i−1]<0, and nums[i]<0, then product[i]>0.
// So we have two states product[i]<0 and product[i]>0.
pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut max = i32::MIN;
    let mut product = 1;
    let mut change_sign = 1;
    for n in nums.into_iter() {
        product *= n;
        max = max.max(product);

        if product < 0 && change_sign < 0 {
            max = max.max(product / change_sign);
        }

        if n < 0 && change_sign > 0 {
            change_sign = product;
        } else if n == 0 {
            product = 1;
            change_sign = 1;
        }
    }
    max
}
pub fn main() {
    let nums = vec![2, 3, -2, 4];
    assert_eq!(max_product(nums), 6);
    let nums = vec![-2, 0, -1];
    assert_eq!(max_product(nums), 0);
    let nums = vec![-3, -1, -1];
    assert_eq!(max_product(nums), 3);
    let nums = vec![0, 2];
    assert_eq!(max_product(nums), 2);
    print_pass(NAME, LINK)
}
