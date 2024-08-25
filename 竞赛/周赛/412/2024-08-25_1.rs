struct Solution;

impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        for _ in 0..k {
            let mut min = 0;
            for j in 0..nums.len() {
                if nums[min] > nums[j] {
                    min = j;
                }
            }
            nums[min] *= multiplier;
        }
        nums
    }
}