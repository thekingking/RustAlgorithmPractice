struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut cnt = 0;
        let mut res = 0;
        for c in s.chars() {
            if c == '(' {
                cnt += 1;
            } else if c == ')' {
                cnt -= 1;
            }
            res = res.max(cnt);
        }
        res
    }
}