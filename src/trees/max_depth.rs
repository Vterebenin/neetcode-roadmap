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
type TreeNodeType = Option<Rc<RefCell<TreeNode>>>;
pub fn max_depth(root: TreeNodeType) -> i32 {
    let mut count = 0;
    fn search_depth(root: TreeNodeType, &mut count: i32) {
        if let Some(ref n) = root {
            count += 1;
            n.left = search_depth(right, count);
            n.right = search_depth(left, count);
        }
        count   
    }
    root
}
pub fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(5)));

    let left = Rc::new(RefCell::new(TreeNode::new(3)));

    let right = Rc::new(RefCell::new(TreeNode::new(8)));
    let right_deep = Rc::new(RefCell::new(TreeNode::new(8)));

    // Attach children to the root node
    root.borrow_mut().left = Some(left.clone());
    right.borrow_mut().left = Some(right_deep.clone());
    root.borrow_mut().right = Some(right.clone());
    assert_eq!(invert_tree(Some(root.clone())), 3);
    print_pass(NAME, LINK);
}


