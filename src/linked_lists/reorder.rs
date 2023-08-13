use crate::utils::print_pass;

const NAME: &str = "reorder-list";
const LINK: &str = "https://leetcode.com/problems/reorder-list/";

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
pub fn reorder_list(head: &mut Option<Box<ListNode>>){
    let mut stack = vec![];
    let mut fast = head.take();
    while let Some(mut n) = fast.take() {
        fast = n.next.take();
        stack.push(Some(n));
    }
    let mut new_head = None;
    let mut new_tail = &mut new_head;

    let len = stack.len();
    let half = if len % 2 == 1 { len / 2 } else { (len - 1) / 2 };
    for i in 0..=half {
        let mut h = stack[i].take();
        let t = stack[len - 1 - i].take();
        h = h.map(|mut x| {
            x.next = t;
            x
        });
        println!("{:?}", h);
        let node = new_tail.insert(h.take().unwrap());
        if node.next.is_some() {
            new_tail = &mut node.next.as_mut().unwrap().next;
        } else {
            new_tail = &mut node.next;
        }
    }
    *head = new_head;
}

pub fn main() {
    let mut list1 = Some(Box::new(ListNode::new(4)));
    list1 = Some(Box::new(list1.unwrap().push_left(3)));
    list1 = Some(Box::new(list1.unwrap().push_left(2)));
    list1 = Some(Box::new(list1.unwrap().push_left(1)));
    let mut answer = Some(Box::new(ListNode::new(3)));
    answer = Some(Box::new(answer.unwrap().push_left(2)));
    answer = Some(Box::new(answer.unwrap().push_left(4)));
    answer = Some(Box::new(answer.unwrap().push_left(1)));
    reorder_list(&mut list1);
    assert_eq!(list1, answer);
    print_pass(NAME, LINK)
}
