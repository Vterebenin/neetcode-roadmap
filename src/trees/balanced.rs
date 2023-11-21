use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::print_pass;
use std::cmp::max;

const NAME: &str = "balanced-binary-tree";


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
pub fn is_balanced(root: TreeNodeType) -> bool {
    fn dfs(root: TreeNodeType) -> (bool, i32) {
        if let Some(ref n) = root {
            let (is_balanced_l, ld) = dfs(n.borrow().left.clone());
            let (is_balanced_r, rd) = dfs(n.borrow().right.clone());
            let current_balance = (ld - rd).abs() + 1 <= 2;
            return (is_balanced_l && is_balanced_r && current_balance, max(ld + 1, rd + 1));
        }
        (true, 1)
    }
    let (balanced, _) = dfs(root);
    balanced
}
pub fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(5)));

    let left = Rc::new(RefCell::new(TreeNode::new(3)));

    let right = Rc::new(RefCell::new(TreeNode::new(8)));
    let right_1_left = Rc::new(RefCell::new(TreeNode::new(8)));
    let right_1_right = Rc::new(RefCell::new(TreeNode::new(8)));
    let right_2_left = Rc::new(RefCell::new(TreeNode::new(8)));
    let right_2_right = Rc::new(RefCell::new(TreeNode::new(8)));

    // Attach children to the root node
    root.borrow_mut().left = Some(left.clone());
    right_1_left.borrow_mut().left = Some(right_2_left.clone());
    right_1_left.borrow_mut().right = Some(right_2_right.clone());
    right.borrow_mut().left = Some(right_1_left.clone());
    right.borrow_mut().right = Some(right_1_right.clone());
    root.borrow_mut().right = Some(right.clone());
    assert_eq!(is_balanced(Some(root.clone())), false);
    print_pass(NAME);
}


