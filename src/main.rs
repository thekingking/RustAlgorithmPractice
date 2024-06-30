use std::{collections::HashMap, vec};

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let s = nums.iter().sum::<i32>() - target.abs();
        if s < 0 || s % 2 == 1 {
            return 0;
        }
        let m = s as usize / 2;
        let mut f = vec![0; m + 1];
        f[0] = 1;
        for &x in &nums {
            let x = x as usize;
            for i in (x..=m).rev() {
                f[i] += f[i - x];
            }
        }
        f[m]
    }
}