struct Solution;

impl Solution {
    /// 记忆化搜索
    /// 逆向思维，从最后一个气球往前数，取出最后一个气球后，该气球左右两侧再作同样的操作
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        fn helper(nums: &Vec<i32>, dp: &mut Vec<Vec<i32>>, left: usize, right: usize) -> i32 {
            if left >= right - 1 {
                return 0;
            }
            if dp[left][right] != 0 {
                return dp[left][right];
            }
            let mut ans = 0;
            for i in (left + 1)..right {
                ans = ans.max(nums[left] * nums[i] * nums[right] + helper(nums, dp, left, i) + helper(nums, dp, i, right));
            }
            dp[left][right] = ans;
            ans
        }
        nums.insert(0, 1);
        nums.push(1);
        let len = nums.len();
        let mut dp = vec![vec![0; len]; len];
        helper(&nums, &mut dp, 0, len - 1)
    }
}
