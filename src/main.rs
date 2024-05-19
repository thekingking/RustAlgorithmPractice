
use std::{collections::HashMap, hash::Hash};



fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        let mut n = nums[0];
        let mut len = 0;
        let mut nums = nums;
        while n != 0 {
            n /= 10;
            len += 1;
        }
        let mut ans:i64 = 1;
        for _ in 0..len {
            let mut arr = vec![0; 10];
            for i in 0..nums.len() {
                arr[nums[i] as usize % 10] += 1;
            }
            for i in 0..10 {
                if arr[i] != 0 {
                    ans *= arr[i];
                }
            }
        }
        ans
    }
}