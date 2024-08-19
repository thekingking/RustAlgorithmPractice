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
        let mut dp = vec![vec![0; n + 2]; 2];
        dp[0][2] = 1;
        dp[1][2] = 1;
        let mut sum = 0;
        for i in 1..n {
            dp[0][i + 2] = (dp[0][i + 1] + dp[1][i + 1]) % MOD;
            dp[1][i + 2] = (dp[0][i + 1] + dp[0][i]) % MOD;
            sum += (sum + dp[0][i + 1] + dp[1][i + 1]) % MOD;
        }
        ((dp[0][n + 1] + dp[1][n + 1] + sum) % MOD) as i32
    }
}