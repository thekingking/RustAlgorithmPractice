use std::collections::HashMap;
struct Solution;

impl Solution {
    /// 前缀和，前缀和记录当前前缀和pre对k的余数，当下一次出现相同余数时，他们之间的数之和即为k整除
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = HashMap::new();
        let mut ans = 0;
        // 初始化0，刚好被整除初始化为1
        let mut pre = 0;
        for x in nums {
            cnt.entry(pre).and_modify(|x| *x += 1).or_insert(1);
            // 将前缀和余数转为正数
            pre = ((pre + x) % k + k) % k;
            ans += cnt.get(&pre).unwrap_or(&0);
        }
        ans
    }
}