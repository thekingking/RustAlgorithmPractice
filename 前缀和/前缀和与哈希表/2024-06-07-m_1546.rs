use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        let mut cnt = HashMap::new();
        let mut ans = 0;
        let mut sum = 0;
        for (i, x) in nums.iter().enumerate() {
            cnt.insert(sum, i);
            sum += x;
            if cnt.get(&(sum - target)).is_some() {
                ans += 1;
                cnt.clear();
                sum = 0;
            }
        }
        ans
    }
}