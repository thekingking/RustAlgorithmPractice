use std::collections::HashMap;

struct Solution;

impl Solution {
    // 看了下提示，之后有空或者明天再看题解
    pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut indices = HashMap::new();
        for i in 0..nums.len() {
            indices.entry(nums[i]).and_modify(|list: &mut Vec<usize>| list.push(i)).or_insert(vec![i]);
        }
        let mut max = 0;
        for v in indices.values() {
            let mut left = 0;
            let mut right = 0;
            while right < v.len() {
                while (v[right] - v[left]) - (right - left) > k as usize && left <= right {
                    left += 1;
                }
                if right - left + 1 > max {
                    max = right - left + 1;
                }
                right += 1;
            }
        }
        max as i32 + 1
    }
}