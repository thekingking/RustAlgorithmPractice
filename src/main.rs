use std::collections::HashMap;
fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let n = nums.len();
        for i in 0..n {
            while ans.len() > 0 && (n - i + ans.len()) as i32 > k && *ans.last().unwrap() > nums[i] {
                ans.pop();
            }
            ans.push(nums[i]);
        }
        ans.truncate(k as usize);
        ans
    }
}