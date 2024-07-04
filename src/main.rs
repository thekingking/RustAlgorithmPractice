use std::{cmp::max, collections::{BinaryHeap, HashMap}, vec};

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn minimum_moves(nums: Vec<i32>, k: i32, max_changes: i32) -> i64 {
        let mut pos = Vec::new();
        let mut c = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                continue;
            }
            pos.push(i as i64);
            c = std::cmp::max(c, 1);
            if i > 0 && nums[i - 1] == 1 {
                if i > 1 && nums[i - 2] == 1 {
                    c = 3;
                }
                c = std::cmp::max(c, 2);
            }
        }
        c = std::cmp::min(c, k);
        if max_changes >= k - c {
            return (std::cmp::max(c - 1, 0) + (k - c) * 2) as i64;
        }

        // 前缀和
        let n = pos.len();
        let mut sum = vec![0; n + 1];
        for i in 0..n {
            sum[i + 1] = sum[i] + pos[i];
        }

        let size = (k - max_changes) as i64;
        let n = n as i64;
        let mut ans = i64::MAX;
        for right in size..=n {
            let left = right - size;
            let mid = (left + right) / 2;
            let index = pos[mid as usize];
            let s1 = index * (mid - left) - (sum[mid as usize] - sum[left as usize]);
            let s2 = sum[right as usize] - sum[mid as usize] - index * (right - mid);
            ans = std::cmp::min(ans, s1 + s2);
        }
        ans + max_changes as i64 * 2
    }
}