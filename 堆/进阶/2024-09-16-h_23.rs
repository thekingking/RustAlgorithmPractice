struct Solution;

// Definition for singly-linked list.
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
impl Solution {
  pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    let mut heap = BinaryHeap::new();
    let mut head: Option<Box<ListNode>> = None;
    let mut p = &mut head;
    for mut list in lists {
      while let Some(node) = list {
        heap.push(Reverse(node.val));
        list = node.next;
      }
    }
    while let Some(val) = heap.pop() {
      p = &mut p.insert(Box::new(ListNode::new(val.0))).next;
    }
    head
  }
}