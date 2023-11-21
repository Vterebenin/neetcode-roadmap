use crate::utils::print_pass;

const NAME: &str = "min-stack";


struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: vec![],
            min_stack: vec![],
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        let last_min = self.min_stack.last().unwrap_or(&val);
        self.min_stack.push(val.min(*last_min));
    }
    
    fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }
    
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

pub fn main() {
    let mut obj = MinStack::new();
    obj.push(-3);
    obj.push(3);
    obj.push(-1);
    obj.push(4);
    obj.pop();
    let ret_3: i32 = obj.top();
    let ret_4: i32 = obj.get_min();
    assert_eq!(ret_3, -1);
    assert_eq!(ret_4, -3);
    print_pass(NAME)
}
