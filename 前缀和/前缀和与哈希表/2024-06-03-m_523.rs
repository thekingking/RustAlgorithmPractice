struct Solution;

impl Solution {
    /// 跟974题差不多，同样前缀和加哈希表，哈希表存储余数最小坐标，利用同余定理进行判断
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut cnt = HashMap::new();
        let mut pre_sum = 0;
        let mut pre_index = -1;
        for x in nums {
            cnt.entry(pre_sum).or_insert(pre_index);
            pre_sum = (pre_sum + x) % k;
            pre_index += 1;
            if let Some(index) = cnt.get(&pre_sum) {
                if pre_index - index >= 2 {
                    return true;
                }
            }
        }
        false
    }
}