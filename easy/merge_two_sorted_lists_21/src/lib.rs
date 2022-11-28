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

fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if list1.is_none() {
        return list2;
    }
    if list2.is_none() {
        return list1;
    }

    let node1 = list1.as_mut().unwrap();
    let node2 = list2.as_mut().unwrap();
    let ans: Box<ListNode>;

    if node1.val > node2.val {
        ans = Box::new(*node2);
    } else {
        ans = *node1;
    }

    let mut ans_ref: Box<ListNode> = ans;
    while node1.next.is_none() == false {
        if node2.next.as_mut().is_none() == true {
            return Some(ans_ref);
        }
        if node1.next.as_mut().unwrap().val > node2.next.as_mut().unwrap().val {
            ans_ref.next = Some(*node2);
        }
    }

    list1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        
    }
}
