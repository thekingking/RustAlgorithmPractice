use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut cnt = HashMap::new();
        let mut sum = 0;
        let mut ans = 0;
        for (i, &x) in hours.iter().enumerate() {
            cnt.entry(sum).or_insert(i);
            sum += if x > 8 { 1 } else { -1 };
            if sum > 0 {
                ans = i + 1;
            } else {
                ans = ans.max(i + 1 - cnt.get(&(sum - 1)).unwrap_or(&(i + 1)));
            }
        }
        ans as i32
    }
}
