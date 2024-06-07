use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 1;
        let score = nums[0] + nums[1];
        let mut i = 2;
        while i + 1 < nums.len() {
            if nums[i] + nums[i + 1] == score {
                ans += 1;
            } else {
                break;
            }
            i += 2;
        }
        ans
    }
}
