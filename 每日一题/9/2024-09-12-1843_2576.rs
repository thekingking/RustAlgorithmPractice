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