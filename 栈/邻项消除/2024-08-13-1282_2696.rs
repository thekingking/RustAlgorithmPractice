struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut res = Vec::new();
        for &c in s.as_bytes() {
            if let Some(&s) = res.last() {
                if s == b'A' && c == b'B' || s == b'C' && c == b'D' {
                    res.pop();
                } else {
                    res.push(c);
                }
            } else {
                res.push(c);
            }
        }
        res.len() as i32
    }
}
