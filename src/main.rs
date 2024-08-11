use std::collections::HashMap;


fn main() {
    println!("hello, world");
}

struct Solution;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums2.len() + 1];
        for i in 0..nums1.len() {
            let mut pre = 0;
            for j in 0..nums2.len() {
                let tmp = dp[j + 1];
                dp[j + 1] = if nums1[i] == nums2[j] { pre + 1 } else { dp[j].max(dp[j + 1]) };
                pre = tmp;
            }
        }
        dp[nums2.len()]
    }
}

