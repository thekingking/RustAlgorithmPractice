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

struct Solution;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dict = std::collections::HashSet::new();
        for x in nums {
            dict.insert(x);
        }

        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;
        while let Some(mut node) = head {
            head = node.next.take();
            if !dict.contains(&node.val) {
                cur = cur.next.get_or_insert(node);
            }
        }
        dummy.next
    }
}