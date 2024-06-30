struct Solution;

impl Solution {
    /// 动态规划
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = [[0; 2001]; 2];
        dp[0][1000] = 1;
        let mut sum = 0;
        for (i, &x) in nums.iter().enumerate() {
            sum += x;
            for j in (1000 - sum)..=(1000 + sum) {
                dp[(i + 1) % 2][j as usize] = 0;
                if j + x <= 2000 {
                    dp[(i + 1) % 2][j as usize] += dp[i % 2][(j + x) as usize];
                }
                if j - x >= 0 {
                    dp[(i + 1) % 2][j as usize] += dp[i % 2][(j - x) as usize];
                }
            }
        }
        dp[nums.len() % 2][(1000 + target) as usize]
    }
}