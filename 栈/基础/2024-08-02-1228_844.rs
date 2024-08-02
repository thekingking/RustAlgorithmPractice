struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut cnt1 = Vec::new();
        let mut cnt2 = Vec::new();
        for &b in s.as_bytes() {
            if b != b'#' {
                cnt1.push(b);
            } else {
                cnt1.pop();
            }
        }
        for &b in t.as_bytes() {
            if b != b'#' {
                cnt2.push(b);
            } else {
                cnt2.pop();
            }
        }
        return cnt1 == cnt2
    }
}