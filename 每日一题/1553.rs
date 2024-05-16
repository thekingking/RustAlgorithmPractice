use std::collections::HashMap;

struct Solution;

impl Solution {
    // 官方题解
    pub fn min_days(n: i32) -> i32 {
        let mut memo = HashMap::new();
        Self::helper(n, &mut memo)
    }

    fn helper(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if n <= 1 {
            return n;
        }
        if let Some(&days) = memo.get(&n) {
            return days;
        }
        let days_with_two = Self::helper(n / 2, memo) + n % 2 + 1;
        let days_with_three = Self::helper(n / 3, memo) + n % 3 + 1;
        let min_days = days_with_two.min(days_with_three);
        memo.insert(n, min_days);
        min_days
    }
}