use crate::utils::print_pass;

const NAME: &str = "remove-nth-node-from-end-of-list";
const LINK: &str = "https://leetcode.com/problems/remove-nth-node-from-end-of-list/";

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
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut tail = head.clone();
    let mut curr = tail.as_mut();
    for i in 0..n {
        if curr.is_some() {
            curr = curr.next;
        }
    }
    tail
}

pub fn main() {
    let mut list1 = Some(Box::new(ListNode::new(5)));
    list1 = Some(Box::new(list1.unwrap().push_left(3)));

    let mut list2 = Some(Box::new(ListNode::new(4)));
    list2 = Some(Box::new(list2.unwrap().push_left(1)));

    let mut list1 = Some(Box::new(ListNode::new(5)));
    list1 = Some(Box::new(list1.unwrap().push_left(4)));
    list1 = Some(Box::new(list1.unwrap().push_left(3)));
    list1 = Some(Box::new(list1.unwrap().push_left(1)));
    let mut answer = Some(Box::new(ListNode::new(5)));
    answer = Some(Box::new(answer.unwrap().push_left(4)));
    answer = Some(Box::new(answer.unwrap().push_left(1)));
    assert_eq!(remove_nth_from_end(list1, 1), answer);
    print_pass(NAME, LINK)
}
