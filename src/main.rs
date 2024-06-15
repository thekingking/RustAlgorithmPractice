use std::{collections::HashMap, f32::MIN};



fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let m = nums.iter().max().unwrap();
        let mut diff = vec![0; (m + 2) as usize];
        for x in nums.iter() {
            diff[0.max(x - k) as usize] += 1;
            diff[(m + 1).min(x + k + 1) as usize] -= 1;
        }
        let mut res = 0;
        let mut count = 0;
        for x in diff.iter() {
            count += x;
            res = res.max(count);
        }
        res
    }
}
