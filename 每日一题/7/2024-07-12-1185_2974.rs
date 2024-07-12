struct Solution;

impl Solution {
    pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut i = 0;
        while i + 1 < nums.len() {
            let temp = nums[i];
            nums[i] = nums[i + 1];
            nums[i + 1] = temp;
            i += 2;
        }
        nums
    }
}