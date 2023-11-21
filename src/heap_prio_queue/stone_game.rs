use crate::utils::print_pass;
use std::collections::BinaryHeap;

const NAME: &str = "last-stone-weight";



pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(stones.len() + 1);
    for stone in stones.iter() { 
        heap.push(stone.to_owned());
    }
    while heap.len() > 1 {
        let y = heap.pop().unwrap();
        let x = heap.pop().unwrap();
        if x != y {
            heap.push(y - x);
        }
    }
    heap.pop().unwrap_or(0)
}
/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
pub fn main() {
    let stones = vec![2, 7, 4, 1, 8, 1];
    let answer = 1;
    assert_eq!(last_stone_weight(stones), answer);
    print_pass(NAME);
}
