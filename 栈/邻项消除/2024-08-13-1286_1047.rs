struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut res = Vec::new();
        for &c in s.as_bytes() {
            if let Some(&s) = res.last() {
                if s == c {
                    res.pop();
                } else {
                    res.push(c);
                }
            } else {
                res.push(c);
            }
        }
        unsafe {String::from_utf8_unchecked(res)}
    }
}