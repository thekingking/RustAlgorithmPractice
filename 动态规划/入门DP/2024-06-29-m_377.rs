struct Solution;

impl Solution {
    /// 记忆化搜索
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        fn dfs(target: usize, nums: &Vec<i32>, memo: &mut Vec<i32>) -> i32 {
            if target == 0 {
                return 1;
            }
            if memo[target] != -1 {
                return memo[target];
            }
            let mut res = 0;
            for &x in nums {
                let x = x as usize;
                if x <= target {
                    res += dfs(target - x, nums, memo);
                }
            }
            memo[target] = res;
            res
        }
        let target = target as usize;
        let mut memo = vec![-1; target + 1];
        dfs(target, &nums, &mut memo)
    }

    /// 递推
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