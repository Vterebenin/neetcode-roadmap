use crate::utils::print_pass;

const NAME: &str = "partition-equal-subset-sum";


pub fn can_partition(nums: Vec<i32>) -> bool {
    let t_sum = nums.iter().fold(0, |acc, item| acc+item);
    if t_sum % 2 == 1 {
        return false;
    }
    let seek = (t_sum / 2) as usize ;
    let mut table = vec![false; seek + 1];
    table[0] = true;
    for num in nums.iter() {
        let n2 = *num as usize;
        for i in (n2..=seek).rev() {
            let index = i as usize;
            table[index] = table[index] || table[index-n2];
        }
        if table[seek] {
            return true;
        }
    }
    false
}

pub fn main() {
    let nums = vec![1,5,11,5];
    assert_eq!(can_partition(nums), true);
    let nums = vec![1,2,3,5];
    assert_eq!(can_partition(nums), false);
    let nums = vec![1,5,3];
    assert_eq!(can_partition(nums), false);
    let nums = vec![1,2,3,4,5,6,7];
    assert_eq!(can_partition(nums), true);
    print_pass(NAME)
}
