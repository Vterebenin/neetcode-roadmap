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
pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
    let mut prehead = ListNode::new(0);
    let mut cur_node = &mut prehead;
    while let (Some(node1), Some(node2)) = (&list1, &list2) {
        if node1.val < node2.val {
            cur_node.next = list1.take();
            cur_node = cur_node.next.as_mut().unwrap();
            list1 = cur_node.next.take();
        } else {
            cur_node.next = list2.take();
            cur_node = cur_node.next.as_mut().unwrap();
            list2 = cur_node.next.take();
        }
    }
    cur_node.next = list1.or(list2);
    prehead.next
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
