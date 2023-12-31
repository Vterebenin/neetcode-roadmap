use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::print_pass;

const NAME: &str = "binary-tree-level-order-traversal";


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
pub fn level_order(root: TreeNodeType) -> Vec<Vec<i32>> {
    let mut q = VecDeque::new();
    if let Some(r) = root {
        q.push_back(r);
    }
    let mut levels: Vec<Vec<i32>> = vec![];
    while !q.is_empty() {
        let mut level = vec![];
        for _ in 0..q.len() {
            if let Some(curr) = q.pop_front() {
                level.push(curr.borrow().val);
                if let Some(l) = curr.borrow_mut().left.take() {
                    q.push_back(l);
                }
                if let Some(r) = curr.borrow_mut().right.take() {
                    q.push_back(r);
                }
            }
        }
        levels.push(level);
    }
    levels
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
    let answer: Vec<Vec<i32>> = vec![
        vec![5],
        vec![3, 8],
        vec![8, 8],
    ];
    assert_eq!(level_order(Some(root.clone())), answer);
    print_pass(NAME);
}


