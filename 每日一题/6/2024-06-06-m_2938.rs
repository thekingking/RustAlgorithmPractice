struct Solution;

impl Solution {
    /// 贪心，简单题
    pub fn minimum_steps(s: String) -> i64 {
        let mut num = 0;
        let mut ans = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '0' {
                ans += i as i64 - num;
                num += 1;
            }
        }
        ans
    }
}