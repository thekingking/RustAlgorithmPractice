struct Solution;

impl Solution {
    /// 可惜没在周赛做出
    /// 动态规划
    pub fn maximum_total_cost(nums: Vec<i32>) -> i64 {
        let mut dp = vec![[i64::MIN, i64::MIN]; nums.len()];
        dp[0][0] = nums[0] as i64;
        for i in 1..nums.len() {
            dp[i][0] = std::cmp::max(dp[i - 1][0], dp[i - 1][1]) + nums[i] as i64;
            dp[i][1] = dp[i - 1][0] - nums[i] as i64;
        }
        std::cmp::max(dp[nums.len() - 1][0], dp[nums.len() - 1][1])
    }
}