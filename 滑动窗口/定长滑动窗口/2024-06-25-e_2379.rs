
struct Solution;

impl Solution {
    /// 滑动窗口
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let byte = blocks.as_bytes();
        let mut num = 0;
        let mut m = 0;
        let mut index = 0;
        let mut res = 100;
        let n = blocks.len();
        while index < n {
            while index < n && num < k {
                if byte[index] == b'W' {
                    m += 1;
                }
                num += 1;
                index += 1;
            }
            if num == k {
                res = res.min(m);
            }
            if byte[index - num as usize] == b'W' {
                m -= 1;
            }
            num -= 1;
        }
        res
    }
}