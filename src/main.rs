use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = i32::MAX;
        for i in 0..nums.len() {
            let mut cnt = nums[i];
            ans = ans.min(i32::abs(k - cnt));
            for j in (0..i).rev() {
                cnt &= nums[j];
                ans = ans.min(i32::abs(k - cnt));
            }
        }
        ans
    }
}