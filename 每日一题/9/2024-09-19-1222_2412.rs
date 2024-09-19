struct Solution;

impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let mut res = 0;
        let mut pre = b'z';
        let mut num = 1;
        for &c in s.as_bytes() {
            if c == pre + 1 {
                num += 1;
            } else {
                num = 1;
            }
            res = res.max(num);
            pre = c;
        }
        res
    }
}