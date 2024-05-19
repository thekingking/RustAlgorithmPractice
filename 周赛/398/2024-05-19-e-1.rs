struct Solution;

impl Solution {
    // 周赛第一题
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return true;
        }
        let mut prev = 0;
        for cur in 1..nums.len() {
            if (nums[prev] + nums[cur]) % 2 == 0 {
                return false;
            } 
            prev += 1;
        }
        return true;
    }
}