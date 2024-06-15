struct Solution;

impl Solution {
    /// 前缀和+哈希表
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut sum = 0;
        for &x in nums.iter() {
            sum = (sum + x) % p;
        }
        if sum == 0 {
            return 0;
        }
        let mut cnt = std::collections::HashMap::new();
        let mut pre = 0;
        let mut ans = nums.len();
        for (i, &x) in nums.iter().enumerate() {
            cnt.insert(pre, i);
            pre = (pre + x) % p;
            if cnt.get(&((pre - sum + p) % p)).is_some() {
                ans = std::cmp::min(ans, i + 1 - *cnt.get(&((pre - sum + p) % p)).unwrap());
            }
        }
        if ans == nums.len() { -1 } else { ans as i32 }
    }
}
