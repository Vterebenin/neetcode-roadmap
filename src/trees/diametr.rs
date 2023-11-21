use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::print_pass;
use std::cmp::max;

const NAME: &str = "diameter-of-binary-tree";


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
pub fn diameter_of_binary_tree(root: TreeNodeType) -> i32 {
    fn dfs(root: TreeNodeType) -> (i32, i32) {
        if let Some(ref n) = root {
            let ld = dfs(n.borrow().left.clone());
            let rd = dfs(n.borrow().right.clone());
            return (max(ld.0, rd.0) + 1, max(ld.1, max(rd.1, ld.0 + rd.0)));
        }
        (0, 0)
    }
    dfs(root).0
}
pub fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(5)));

    let left = Rc::new(RefCell::new(TreeNode::new(3)));

    let right = Rc::new(RefCell::new(TreeNode::new(8)));
    let right_deep = Rc::new(RefCell::new(TreeNode::new(8)));
    let right_right_deep = Rc::new(RefCell::new(TreeNode::new(8)));

    // Attach children to the root node
    root.borrow_mut().left = Some(left.clone());
    right.borrow_mut().left = Some(right_deep.clone());
    right.borrow_mut().right = Some(right_right_deep.clone());
    root.borrow_mut().right = Some(right.clone());
    assert_eq!(diameter_of_binary_tree(Some(root.clone())), 3);
    print_pass(NAME);
}


