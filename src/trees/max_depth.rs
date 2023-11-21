use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::print_pass;

const NAME: &str = "maximum-depth-of-binary-tree";


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
    let count = 0;
    fn search_depth(root: TreeNodeType, count: i32) -> i32 {
        let mut local_count = count.clone();
        if let Some(ref n) = root {
            local_count += 1;
            let count_left = search_depth(n.borrow().left.clone(), local_count);
            let count_right = search_depth(n.borrow().right.clone(), local_count);
            local_count = count_left.max(count_right);
        }
        local_count   
    }
    search_depth(root, count)
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
    assert_eq!(max_depth(Some(root.clone())), 3);
    print_pass(NAME);
}


