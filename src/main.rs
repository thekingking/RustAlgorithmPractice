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
  pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
    let n = prize_positions.len();
    if k * 2 + 1 >= prize_positions[n - 1] - prize_positions[0] {
      return n as _;
    }
    let mut ans = 0;
    let mut left = 0;
    let mut mx = vec![0; n + 1];
    for (right, &p) in prize_positions.iter().enumerate() {
      while p - prize_positions[left] > k {
        left += 1;
      }
      ans = ans.max(mx[left] + right - left + 1);
      mx[right + 1] = mx[right].max(right - left + 1);
    }
    ans as _
  }
}