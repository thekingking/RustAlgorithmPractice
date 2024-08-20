impl Solution {
    /// 空间优化
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

    pub fn check_record(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;
        if n == 1 {
            return 3;
        }
        let mut dp = vec![vec![0; n]; 3];
        dp[0][0] = 1;
        dp[1][0] = 1;
        dp[2][0] = 0;
        let mut sum = 0;
        for i in 1..n {
            dp[0][i] = (dp[0][i - 1] + dp[1][i - 1] + dp[2][i - 1]) % MOD;
            dp[1][i] = dp[0][i - 1];
            dp[2][i] = dp[1][i - 1];
        }
        for i in 0..=n {
            if i == 0 {
                sum = (sum + dp[0][n - i - 2] + dp[1][n - i - 2] + dp[2][n - i - 2]) % MOD;
            } else if i == n - 1 {
                sum = (sum + dp[0][i - 1] + dp[1][i - 1] + dp[2][i - 1]) % MOD;
            } else if i == n {
                sum = (sum + dp[0][i - 1] + dp[1][i - 1] + dp[2][i - 1]) % MOD;
            } else {
                sum = (sum + (dp[0][i - 1] + dp[1][i - 1] + dp[2][i - 1]) * (dp[0][n - i - 2] + dp[1][n - i - 2] + dp[2][n - i - 2])) % MOD;
            }
        }
        sum as i32
    }
}