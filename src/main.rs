use std::i32;




fn main() {
    println!("hello, world");
}

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
  pub fn max_num_of_marked_indices(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut right = nums.len() - 1;
    for left in (0..nums.len() / 2).rev() {
      if nums[left] * 2 <= nums[right] {
        right -= 1;
      }
    } 
    (nums.len() - right - 1) as i32 * 2
  }
}