use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::print_pass;

const NAME: &str = "lowest-common-ancestor-of-a-binary-search-tree";
const LINK: &str = "https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/";

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

pub fn lowest_common_ancestor(root: TreeNodeType, p: TreeNodeType, q: TreeNodeType) -> TreeNodeType {
    let mut vp = p.as_ref().unwrap().borrow().val;
    let mut vq = q.as_ref().unwrap().borrow().val;
    // mem swap if incorrect values
    if vp > vq { std::mem::swap(&mut vp, &mut vq) }
    match root {
        x if x == p => { p },
        x if x == q => { q },
        Some(node) => {
            let v = node.borrow().val;
            if vp < v && v < vq { return Some(node); }
            if vq < v { lowest_common_ancestor(node.borrow().left.clone(), p, q) }
            else { lowest_common_ancestor(node.borrow().right.clone(), p, q) }
        },
        None => None
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
    assert_eq!(lowest_common_ancestor(Some(root.clone()), Some(l2.clone()), Some(l1.clone())), Some(left));
    print_pass(NAME, LINK);
}


