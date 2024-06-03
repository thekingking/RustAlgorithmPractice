struct Solution;

impl Solution {
    /// 前缀和与总和，计算两侧总和，左侧都小于nums[i]，右侧都大于nums[i]
    /// 双百
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let sum = nums.iter().sum::<i32>();
        let n = nums.len() as i32;
        let mut pre_sum = 0;
        let mut ans = vec![0; nums.len()];
        for (i, x) in nums.iter().enumerate() {
            ans[i] = sum - pre_sum * 2 + x * (2 * i as i32 - n);
            pre_sum += x;
        }
        ans
    }
}