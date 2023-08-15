use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::print_pass;

const NAME: &str = "invert-binary-tree";
const LINK: &str = "https://leetcode.com/problems/invert-binary-tree/";

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
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(ref n) = root {
        let left = n.borrow().left.clone();
        let right = n.borrow().right.clone();

        let mut n = n.borrow_mut();
        n.left = invert_tree(right);
        n.right = invert_tree(left);
    }
    root
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
    assert_eq!(invert_tree(Some(root.clone())), Some(answer.clone()));
    print_pass(NAME, LINK);
}


