use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::print_pass;

const NAME: &str = "same-tree";
const LINK: &str = "https://leetcode.com/problems/same-tree/";

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
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
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            let p = p.borrow();
            let q = q.borrow();
            p.val == q.val
                && is_same_tree(p.left.clone(), q.left.clone())
                && is_same_tree(p.right.clone(), q.right.clone())
        }
        _ => false,
    }
}
pub fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(5)));
    let answer = root.clone();

    let left = Rc::new(RefCell::new(TreeNode::new(3)));

    let right = Rc::new(RefCell::new(TreeNode::new(8)));

    // Attach children to the root node
    root.borrow_mut().left = Some(left.clone());
    root.borrow_mut().right = Some(right.clone());
    answer.borrow_mut().left = Some(right.clone());
    answer.borrow_mut().right = Some(left.clone());
    assert_eq!(is_same_tree(Some(root.clone()), Some(answer.clone())), true);
    print_pass(NAME, LINK);
}


