use std::collections::HashMap;

struct Solution;

impl Solution {
    // 跟930有点像，使用前缀和加哈希表方法代码是一样的，但是由于nums[i]与k可以为负数，这个题不能使用滑动窗口
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = HashMap::new();
        let mut ans = 0;
        let mut sum = 0;
        for i in 0..nums.len() {
            cnt.entry(sum).and_modify(|x| *x += 1).or_insert(1);
            sum += nums[i];
            ans += cnt.get(&(sum - k)).unwrap_or(&0);
        }
        ans
    }
}