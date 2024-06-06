use std::collections::HashMap;

struct Solution;

impl Solution {
    /// 异或和、前缀和、哈希表
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let mut cnt = HashMap::new();
        let mut orx = 0;
        let mut ans = 0;
        for x in nums {
            cnt.entry(orx).and_modify(|x| *x += 1).or_insert(1);
            orx ^= x;
            ans += cnt.get(&orx).unwrap_or(&0);
        }
        ans
    }
}
