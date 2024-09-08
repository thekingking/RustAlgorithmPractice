struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        while i < nums.len() && nums[i] < 0 {
            i += 1;
        }
        let mut left = i as i32 - 1;
        let mut right = i as i32;
        let mut res = Vec::new();
        while left >= 0 && right < nums.len() as i32 {
            if nums[left as usize].abs() < nums[right as usize] {
                res.push(nums[left as usize].pow(2));
                left -= 1;
            } else {
                res.push(nums[right as usize].pow(2));
                right += 1;
            }
        }
        while left >= 0 {
            res.push(nums[left as usize].pow(2));
            left -= 1;
        }
        while right < nums.len() as i32 {
            res.push(nums[right as usize].pow(2));
            right += 1;
        }
        res
    }
}