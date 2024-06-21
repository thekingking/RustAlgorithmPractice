struct Solution;

impl Solution {
    /// 排序+滑动窗口
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let k = k as usize;
        let mut res = 100_000;
        if k > nums.len() {
            return nums[nums.len() - 1] - nums[0];
        }
        for i in 0..=(nums.len() - k) {
            res = res.min(nums[i + k - 1] - nums[i]);
        }
        res
    }
}