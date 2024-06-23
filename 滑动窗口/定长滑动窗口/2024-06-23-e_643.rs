struct Solution;

impl Solution {
    /// 滑动窗口
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut all = 0;
        let mut max = i32::MIN;
        let mut num = 0;
        let mut i = 0;
        while i < nums.len() {
            while i < nums.len() && num < k {
                all += nums[i];
                i += 1;
                num += 1;
            }
            max = max.max(all);
            all -= nums[i - k as usize];
            num -= 1;
        }
        max as f64 / k as f64
    }
}