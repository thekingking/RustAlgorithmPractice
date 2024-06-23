struct Solution;

impl Solution {
    /// 滑动窗口，注意i32正溢出
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return nums;
        }
        let mut num = 0;
        let mut sum = 0;
        let mut res = vec![-1; nums.len()];
        let mut i = 0;
        let k = k as usize;
        while i < nums.len() {
            while i < nums.len() && num < 2 * k + 1 {
                sum += nums[i] as i64;
                i += 1;
                num += 1;
            }
            if num == 2 * k + 1 {
                res[i - k - 1] = (sum / (2 * k as i64 + 1)) as i32;
                sum -= nums[i - (2 * k + 1)] as i64;
                num -= 1;
            }
        }
        res
    }
}