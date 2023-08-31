use crate::utils::print_pass;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

const NAME: &str = "kth-largest-element-in-a-stream";
const LINK: &str = "https://leetcode.com/problems/kth-largest-element-in-a-stream/";


pub struct KthLargest {
    size: usize,
    heap: BinaryHeap<Reverse<i32>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let size = k as usize;
        let mut heap = BinaryHeap::with_capacity(size + 1);
        for n in nums { 
            heap.push(Reverse(n));
            if heap.len() > size {
                heap.pop();
            }
        }
        Self { 
            size, 
            heap,
        }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.size {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
pub fn main() {
    let nums = vec![4, 5, 8, 2];
    let k = 3;
    let mut obj = KthLargest::new(k, nums);
    assert_eq!(obj.add(3), 4);
    assert_eq!(obj.add(5), 5);
    assert_eq!(obj.add(10), 5);
    assert_eq!(obj.add(9), 8);
    assert_eq!(obj.add(4), 8);
    print_pass(NAME, LINK)
}
