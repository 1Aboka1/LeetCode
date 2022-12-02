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
}

struct Solution {
    a: String,
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr = head;

        while let Some(mut boxed_node) = curr {
            let mut next = boxed_node.next.take();
            boxed_node.next = prev.take();
            prev = Some(boxed_node);
            curr = next.take();
        }

        prev
    }
}
