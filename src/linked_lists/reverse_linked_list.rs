use crate::utils::print_pass;

const NAME: &str = "longest-repeating-character-replacement";
const LINK: &str = "https://leetcode.com/problems/longest-repeating-character-replacement/";

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
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;
    while let Some(mut node) = curr {
        curr = node.next;
        node.next = prev;
        prev = Some(node);
    }
    prev
}

pub fn main() {
    let mut list = Some(Box::new(ListNode::new(5)));
    list = Some(Box::new(list.unwrap().push_left(3)));

    let mut reversed = Some(Box::new(ListNode::new(3)));
    reversed = Some(Box::new(reversed.unwrap().push_left(5)));
    assert_eq!(reverse_list(list), reversed);

    let mut list = Some(Box::new(ListNode::new(1)));
    list = Some(Box::new(list.unwrap().push_left(2)));

    let mut reversed = Some(Box::new(ListNode::new(2)));
    reversed = Some(Box::new(reversed.unwrap().push_left(1)));
    assert_eq!(reverse_list(list), reversed);
    print_pass(NAME, LINK)
}
