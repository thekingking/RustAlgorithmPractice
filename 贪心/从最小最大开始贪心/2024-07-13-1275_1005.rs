struct Solution;

impl Solution {
    /// 排序 + 贪心
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut num = 0;
        let mut i = 0;
        while i < nums.len() && num != k && nums[i] < 0 {
            nums[i] = -nums[i];
            i += 1;
            num += 1;
        }
        if i == nums.len() || i != 0 && nums[i] > nums[i - 1] {
            i -= 1;
        }
        if (k - num) % 2 == 1 {
            nums[i] = -nums[i];
        }
        nums.iter().sum()
    }
}