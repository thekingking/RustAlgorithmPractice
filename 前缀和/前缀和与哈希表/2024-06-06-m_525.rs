use std::collections::HashMap;

struct Solution;

impl Solution {
    /// 前缀和、哈希表，化0为-1，记录1和-1出现之和
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut cnt = HashMap::new();
        let mut pre_sum = 0;
        let mut pre_index = -1;
        let mut ans = 0;
        for x in nums {
          cnt.entry(pre_sum).or_insert(pre_index);
          pre_sum += if x == 1 { 1 } else { -1 };
          pre_index += 1;
          ans = ans.max(pre_index - *cnt.get(&pre_sum).unwrap_or(&pre_index));
        }
        ans
    }
}
