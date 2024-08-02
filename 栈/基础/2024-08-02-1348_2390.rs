struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut res = Vec::new();
        for c in s.chars() {
            if c == '*' {
                res.pop();
            } else {
                res.push(c);
            }
        }
        res.into_iter().collect::<String>()
    }
}