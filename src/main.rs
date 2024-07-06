use std::{cmp::max, collections::{BinaryHeap, HashMap}, vec};

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        let mut cnt = 0;
        for i in 0..nums.len() {
            if i > 0 && nums[i] != nums[i - 1] {
                cnt += 1;
            } else {
                cnt = 1;
            }
            res += cnt;
        }
        res
    }
}