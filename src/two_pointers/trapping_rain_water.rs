use crate::utils::print_pass;

const NAME: &str = "two-sum-ii-input-array-is-sorted";

pub fn trap(height: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, height.len() - 1);
    let mut pool_height = 0;
    let mut trapped = 0;
    
    while left < right {
        pool_height = pool_height.max(height[left].min(height[right]));
        
        if height[left] <= height[right] {
            trapped += 0.max(pool_height - height[left]);
            left += 1;
        } else {
            trapped += 0.max(pool_height - height[right]);
            right -= 1;
        }
    }
    
    trapped
}

pub fn main() {
    let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    let target = 6;
    assert_eq!(trap(height), target);
    print_pass(NAME)
}
