use std::collections::HashMap;

struct Solution;

impl Solution {
    /// 前缀和与哈希表
    /// 记录前缀和为sum，遇到数字则+1，遇到字母则-1，将以前缀和为key，对应前缀和的最小下标为value，存放在哈希表中
    /// 当前前缀和为sum，若哈希表中存在与当前前缀和相同的前缀和，即两个前缀和下标范围内数字和字母数量相同，两个下标相减即为范围，求最大范围即可。
    /// left指向当前元素，right指向当前元素的下一个元素（Rust下标为usize，所以从0开始，left=0,right=0，即当前无元素，范围长度为0）
    pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
        let mut cnt = HashMap::new();
        let mut sum = 0;
        let mut left = 0;
        let mut right = 0;
        for (r, x) in array.iter().enumerate() {
            cnt.entry(sum).or_insert(r);
            let &bit = x.as_bytes().first().unwrap();
            if bit >= b'0' && bit <= b'9' {
                sum += 1;
            } else {
                sum -= 1;
            }
            let &l = cnt.get(&sum).unwrap_or(&(r + 1));
            if r + 1 - l > right - left {
                right = r + 1;
                left = l;
            }
        }
        array[left..right].to_owned()
    }
}
