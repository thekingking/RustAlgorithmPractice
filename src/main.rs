use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let sum = nums.iter().sum::<i32>();
        let n = nums.len() as i32;
        let mut pre_sum = 0;
        let mut ans = vec![0; nums.len()];
        for (i, x) in nums.iter().enumerate() {
            ans[i] = x * i as i32 - pre_sum + (sum - pre_sum - x) - x * (n - i as i32 - 1);
            pre_sum += x;
        }
        ans
    }
}