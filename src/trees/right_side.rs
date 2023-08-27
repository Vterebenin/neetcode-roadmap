use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::print_pass;

const NAME: &str = "binary-tree-right-side-view";
const LINK: &str = "https://leetcode.com/problems/binary-tree-right-side-view/";

type TreeNodeType = Option<Rc<RefCell<TreeNode>>>;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    val: i32,
    left: TreeNodeType,
    right: TreeNodeType,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut q = VecDeque::new();
    if let Some(r) = root {
        q.push_back(r);
    }
    let mut right_side_items: Vec<i32> = vec![];
    while !q.is_empty() {
        let q_size = q.len();
        for index in 0..q_size {
            if let Some(curr) = q.pop_front() {
                if let Some(l) = curr.borrow_mut().left.take() {
                    q.push_back(l);
                }
                if let Some(r) = curr.borrow_mut().right.take() {
                    q.push_back(r);
                }
                if index == q_size - 1 {
                    right_side_items.push(curr.borrow().val);
                }
            }
        }
    }
    right_side_items
}
pub fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(5)));

    let left = Rc::new(RefCell::new(TreeNode::new(3)));

    let right = Rc::new(RefCell::new(TreeNode::new(8)));

    // Attach children to the root node
    root.borrow_mut().left = Some(left.clone());
    root.borrow_mut().right = Some(right.clone());
    assert_eq!(right_side_view(Some(root.clone())), [5, 8]);
    print_pass(NAME, LINK);
}


