struct Solution;

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut cnt = vec![0; n];
        let mut res = vec![0; n - k + 1];
        for i in 1..n {
            if nums[i] == nums[i - 1] + 1 {
                cnt[i] = cnt[i - 1];
            } else {
                cnt[i] = cnt[i - 1] + 1;
            }
        }
        for i in 0..=n - k {
            if cnt[i] == cnt[i + k - 1] {
                res[i] = nums[i + k - 1];
            } else {
                res[i] = -1;
            }
        }
        res
    }
}
