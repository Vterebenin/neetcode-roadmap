use crate::utils::print_pass;

const NAME: &str = "plus-one";


pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for x in digits.iter_mut().rev() {
        match *x == 9 {
            true => *x = 0,
            false => {
                *x += 1;
                return digits;
            }
        }
    }
    digits.insert(0, 1);
    digits
}

pub fn main() {
    let nums = vec![9];
    assert_eq!(plus_one(nums), vec![1, 0]);
    let nums = vec![1, 2, 3];
    assert_eq!(plus_one(nums), vec![1, 2, 4]);
    let nums = vec![9,8,7,6,5,4,3,2,1,0];
    assert_eq!(plus_one(nums), vec![9,8,7,6,5,4,3,2,1,1]);
    print_pass(NAME)
}
