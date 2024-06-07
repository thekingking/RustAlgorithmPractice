struct Solution;

impl Solution {
    /// 简单题，遍历一遍就行了
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 1;
        let score = nums[0] + nums[1];
        let mut i = 2;
        while i + 1 < nums.len() {
            if nums[i] + nums[i + 1] == score {
                ans += 1;
            } else {
                break;
            }
            i += 2;
        }
        ans
    }
}
