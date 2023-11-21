use crate::utils::print_pass;

const NAME: &str = "two-sum-ii-input-array-is-sorted";

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;
    loop {
        while numbers[left] + numbers[right] > target { 
            right -= 1;
        }
        if numbers[left] + numbers[right] == target {
            break vec![left as i32 + 1, right as i32 + 1];
        } else { 
            left += 1;
        }
    }
}

pub fn main() {
    let items = vec![2, 3, 4];
    let target = 6;
    assert_eq!(two_sum(items, target), vec![1, 3]);
    print_pass(NAME)
}
