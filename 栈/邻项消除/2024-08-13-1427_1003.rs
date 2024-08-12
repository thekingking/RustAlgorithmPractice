struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut res = Vec::new();
        for &c in s.as_bytes() {
            let n = res.len();
            if c == b'c' && n >= 2 && res[n - 1] == b'b' && res[n - 2] == b'a' {
                res.pop();
                res.pop();
            } else {
                res.push(c);
            }
        }
        res.len() == 0
    }
}