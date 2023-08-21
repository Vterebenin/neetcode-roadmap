use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::print_pass;

const NAME: &str = "subtree-of-another-tree";
const LINK: &str = "https://leetcode.com/problems/subtree-of-another-tree/";

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

type NodeRef = Option<Rc<RefCell<TreeNode>>>;
pub fn is_subtree(root: NodeRef, sub_root: NodeRef) -> bool {
    fn is_same_tree(p: &NodeRef, q: &NodeRef) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let TreeNode { left: p_left, right: p_right, val: p_val, } = &*p.borrow();
                let TreeNode { left: q_left, right: q_right, val: q_val, } = &*q.borrow();
                p_val == q_val && is_same_tree(p_left, q_left) && is_same_tree(p_right, q_right)
            }
            _ => false,
        }
    }

    match (root.clone(), sub_root.clone()) {
        (None, _) => false,
        (Some(_), None) => true,
        (Some(root_rc), Some(_)) => {
            let root_node = &*root_rc.borrow();

            if is_same_tree(&root, &sub_root) {
                return true;
            }

            is_subtree(root_node.left.clone(), sub_root.clone()) || 
            is_subtree(root_node.right.clone(), sub_root.clone())
        }
    }
}
pub fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(6)));

    let left = Rc::new(RefCell::new(TreeNode::new(2)));

    let right = Rc::new(RefCell::new(TreeNode::new(8)));
    let l1 = Rc::new(RefCell::new(TreeNode::new(0)));
    let l2 = Rc::new(RefCell::new(TreeNode::new(4)));

    // Attach children to the root node
    left.borrow_mut().left = Some(l1.clone());
    left.borrow_mut().right = Some(l2.clone());
    root.borrow_mut().left = Some(left.clone());
    root.borrow_mut().right = Some(right.clone());
    assert_eq!(is_subtree(Some(root.clone()), Some(left.clone())), true);
    print_pass(NAME, LINK);
}


