use std::{default, fmt::Debug, i32};



fn main() {
    println!("hello, world");
}

struct Solution;

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;
        if n == 1 {
            return 3;
        }
        let mut dp = vec![0; n + 3];
        dp[1] = 1;
        dp[2] = 1;
        let mut sum = 0;
        for i in 0..n {
            dp[i + 3] = (dp[i + 2] + dp[i + 1] + dp[i]) % MOD;
        }
        for i in 0..=n {
            if i == 0 || i == n - 1 {
                sum = (sum + dp[n + 1]) % MOD;
            } else if i == n {
                sum = (sum + dp[n + 2]) % MOD;
            } else {
                sum = (sum + (dp[i + 2]) * (dp[n - i + 1])) % MOD;
            }
        }
        sum as i32
    }
}