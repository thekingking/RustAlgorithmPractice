struct Solution;

impl Solution {
    /// 前缀和 + 哈希表
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let mut cnt = std::collections::HashMap::new();
        let mut pre = 0;
        let mut res = 0;
        for &x in nums.iter() {
            cnt.entry(pre).and_modify(|x| *x += 1).or_insert(1);
            pre = (pre + (x % modulo == k) as i32) % modulo; 
            res += cnt.get(&((pre - k + modulo) % modulo)).unwrap_or(&0);
        }
        res
    }
}