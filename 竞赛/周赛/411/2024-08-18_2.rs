struct Solution;

impl Solution {
    pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
        let n = energy_drink_a.len();
        let mut dp = vec![vec![0; n + 1]; 2];
        dp[0][1] = energy_drink_a[0] as i64;
        dp[1][1] = energy_drink_b[0] as i64;
        for i in 1..n {
            dp[0][i + 1] = std::cmp::max(dp[0][i], dp[1][i - 1]) + energy_drink_a[i] as i64;
            dp[1][i + 1] = std::cmp::max(dp[1][i], dp[0][i - 1]) + energy_drink_b[i] as i64;
        }
        std::cmp::max(dp[0][n], dp[1][n])
    }
}
