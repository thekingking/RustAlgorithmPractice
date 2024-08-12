struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let mut pre = nums[0];
        for &x in &nums[1..] {
            if (x + pre) % 2 == 0 {
                return false;
            }
            pre = x;
        }
        true
    }
}
