use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let (mut left1, mut left2, mut right) = (0, 0, 0);
        let (mut sum1, mut sum2) = (0, 0);
        let mut ret = 0;
        while right < nums.len() {
            sum1 += nums[right];
            while left1 <= right && sum1 > goal {
                sum1 -= nums[left1];
                left1 += 1;
            }
            sum2 += nums[right];
            while left2 <= right && sum2 >= goal {
                sum2 -= nums[left2];
                left2 += 1;
            }
            ret += left2 - left1;
            right += 1;
        }
        ret as i32
    }
}