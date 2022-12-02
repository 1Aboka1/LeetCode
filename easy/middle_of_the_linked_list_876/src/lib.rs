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

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut length = 0;
    let mut head_copy = head;
    let mut head_clone = head_copy.clone();

    while let Some(mut boxed_node) = head_copy {
        length += 1;
        head_copy = boxed_node.next.take();
    }

    let mut i = 1;
    while let Some(mut boxed_node) = head_clone {
        if i == length / 2 + 1 {
            return Some(boxed_node);                
        } else {
            head_clone = boxed_node.next.take();
            i += 1;
        }

    }

    Some(Box::new(ListNode { val: 228, next: None }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 3, next: None }) )})) }));
        let result = middle_node(input);
        let correct_result = Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 3, next: None }) )}));
        assert_eq!(result, correct_result);
    }
}
