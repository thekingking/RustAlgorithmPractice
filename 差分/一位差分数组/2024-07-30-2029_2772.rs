struct Solution;

impl Solution {
    pub fn check_array(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let k = k as usize;
        let mut cnt = vec![0; n];
        let mut sum = 0;
        for i in 0..n {
            sum += cnt[i];
            if i <= n - k {
                if nums[i] - sum < 0 {
                    return false;
                } else if nums[i] - sum > 0 {
                    cnt[i + k] = sum - nums[i];
                    sum = nums[i];
                }
            } else if nums[i] - sum != 0 {
                return false;
            }
        }
        true
    }
}
