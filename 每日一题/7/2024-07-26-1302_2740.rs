struct Solution;

impl Solution {
    /// 贪心
    pub fn find_value_of_partition(mut nums: Vec<i32>) -> i32 {
        let mut res = i32::MAX;
        nums.sort_unstable();
        for i in 0..nums.len() - 1 {
            if res > nums[i + 1] - nums[i] {
                res = nums[i + 1] - nums[i];
            }
        }
        res
    }
}
