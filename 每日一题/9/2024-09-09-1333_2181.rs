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

struct Solution;

impl Solution {
    pub fn merge_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = head.as_mut().unwrap();
        let mut q = p.next.take();
        while let Some(mut node) = q {
            if node.val == 0 {
                if p.val != 0 && node.next.is_some() {
                    p.next = Some(Box::new(ListNode::new(0)));
                    p = p.next.as_mut().unwrap();
                }
            } else {
                p.val += node.val;
            }
            q = node.next.take();
        }
        head
    }   
}