struct Solution;

impl Solution {
    /// 简单定长滑动窗口
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut ans = 0;
        let mut num = 0;
        let b = s.as_bytes();
        while right < b.len() {
            while right - left < k as usize && right < b.len() {
                if b[right] == b'a' || b[right] == b'e' || b[right] == b'i' || b[right] == b'o' || b[right] == b'u' {
                    num += 1;
                }
                right += 1;
            }
            ans = ans.max(num);
            if b[left] == b'a' || b[left] == b'e' || b[left] == b'i' || b[left] == b'o' || b[left] == b'u' {
                num -= 1;
            }
            left += 1;
        }
        ans
    }
}
