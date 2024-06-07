use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut cnt = HashMap::new();
        let mut ans = i64::MIN;
        let mut sum = 0;
        for x in nums {
            cnt.entry(x + k).and_modify(|old| if *old > sum {*old = sum}).or_insert(sum);
            cnt.entry(x - k).and_modify(|old| if *old > sum {*old = sum}).or_insert(sum);
            sum += x as i64;
            if cnt.get(&x).is_none() {
                continue;
            }
            ans = ans.max(sum - cnt.get(&x).unwrap());
        }
        if ans == i64::MIN { 0 } else { ans }
    }
}
