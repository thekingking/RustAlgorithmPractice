struct Solution;
impl Solution {
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        nums.sort();
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut res = 100;
        while left < right {
            res = res.min(nums[left] + nums[right]);
            left += 1;
            right -= 1;
        }
        res as f64 / 2.0
    }
}