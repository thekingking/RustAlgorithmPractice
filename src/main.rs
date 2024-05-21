use std::{collections::HashMap, ops::Sub, vec};
fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    // 官解
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = s.len();
        let mut preSum = vec![0; n];
        let mut sum = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '*' {
                sum += 1;
            }
            preSum[i] = sum;
        }
        let mut left = vec![0; n];
        let mut l = -1;
        for (i, c) in s.chars().enumerate() {
            if c == '|' {
                l = i as i32;
            }
            left[i] = l;
        }
        let mut right = vec![0; n];
        let mut r = -1;
        for (i, c) in s.chars().rev().enumerate() {
            if c == '|' {
                r = (n - i - 1) as i32;
            }
            right[n - i - 1] = r;
        }
        let mut ans = vec![0; queries.len()];
        for (i, q) in queries.iter().enumerate() {
            let x = right[q[0] as usize];
            let y = left[q[1] as usize];
            ans[i] = if x == -1 || y == -1 || x >= y { 0 } else {preSum[y as usize] - preSum[x as usize]};
        }
        ans
    }
}