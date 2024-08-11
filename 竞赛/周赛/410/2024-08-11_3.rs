struct Solution;

impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut dp = vec![[0; 52]; nums.len()];
        for i in 0..52 {
            dp[0][i as usize] = i;
        }
        let mut pre = nums[0];
        for i in 1..nums.len() {
            let s = (nums[i] - pre).max(0) as usize;
            for j in s..=nums[i] as usize {
                dp[i][j + 1] = (dp[i - 1][j + 1 - s] + dp[i][j]) % MOD;
            }
            pre = nums[i];
        }
        dp[nums.len() - 1][*nums.last().unwrap() as usize + 1] as i32
    }
}
