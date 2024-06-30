use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let  target = target as usize;
        let mut dp = vec![0; target + 1];
        dp[0] = 1;
        for i in 1..=target {
            for &x in nums.iter() {
                let x = x as usize;
                if x <= i {
                    dp[i] += dp[i - x];
                }
            }
        }
        dp[target]
    }
}