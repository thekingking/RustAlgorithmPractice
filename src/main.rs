use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![-1; n];
        let mut stack = Vec::new();
        for i in 0..2 * n {
            let x = nums[i % n];
            while let Some(&top) = stack.last() {
                if x <= nums[top] {
                    break;
                }
                res[top] = x;
                stack.pop();
            }
            if i < n {
                stack.push(i);
            }
        }
        res
    }
}