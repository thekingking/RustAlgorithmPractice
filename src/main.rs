use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp = vec![i32::MAX / 2; n + 1];
        dp[0] = 0;
        for i in 0..n {
            let c = cost[i];
            let t = time[i] + 1;
            for j in (0..=n).rev() {
                dp[j] = std::cmp::min(dp[j], dp[std::cmp::max(j as i32 - t, 0) as usize] + c);
            }
        }
        dp[n]
    }
}