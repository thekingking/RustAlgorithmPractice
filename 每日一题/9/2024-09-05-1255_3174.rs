struct Solution;

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut stack = Vec::new();
        for c in s.chars() {
            if c.is_ascii_digit() {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        String::from_iter(stack)
    }
}