use crate::utils::print_pass;

const NAME: &str = "merge-two-sorted-lists";
const LINK: &str = "https://leetcode.com/problems/merge-two-sorted-lists/";

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    #[inline]
    fn push_left(self, x: i32) -> ListNode {
        ListNode {
            val: x,
            next: Some(Box::new(self))
        }
    }
}
pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut r = &mut l1;
    while l2.is_some() {
        if r.is_none() || l2.as_ref()?.val < r.as_ref()?.val {
            std::mem::swap(r, &mut l2);
        }
        r = &mut r.as_mut()?.next;
    }
    l1
}

pub fn main() {
    let mut list1 = Some(Box::new(ListNode::new(5)));
    list1 = Some(Box::new(list1.unwrap().push_left(3)));

    let mut list2 = Some(Box::new(ListNode::new(4)));
    list2 = Some(Box::new(list2.unwrap().push_left(1)));

    let mut answer = Some(Box::new(ListNode::new(5)));
    answer = Some(Box::new(answer.unwrap().push_left(4)));
    answer = Some(Box::new(answer.unwrap().push_left(3)));
    answer = Some(Box::new(answer.unwrap().push_left(1)));
    assert_eq!(merge_two_lists(list1, list2), answer);
    print_pass(NAME, LINK)
}
