use std::{cmp::max, collections::{BinaryHeap, HashMap}, vec};

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let mut s = 0;
        for (i, &x) in nums.iter().enumerate() {
            if s == sum - x - s {
                return i as i32;
            }
            s += x;
        }
        -1
    }
}