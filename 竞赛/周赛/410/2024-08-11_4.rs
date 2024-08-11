struct Solution;

impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut dp = vec![0; 1002];
        for i in 0..52 {
            dp[i as usize] = i;
        }
        let mut pre = nums[0];
        for i in 1..nums.len() {
            let s = (nums[i] - pre).max(0) as usize;
            let mut new_dp = vec![0; 1002];
            for j in s..=nums[i] as usize {
                new_dp[j + 1] = (dp[j + 1 - s] + new_dp[j]) % MOD;
            }
            pre = nums[i];
            dp = new_dp;
        }
        dp[*nums.last().unwrap() as usize + 1] as i32
    }
}
