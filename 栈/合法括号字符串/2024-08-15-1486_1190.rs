struct Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = Vec::new();
        for c in s.chars() {
            if c == ')' {
                let mut cnt = Vec::new();
                while let Some(c) = stack.pop() {
                    if c == '(' {
                        break;
                    }
                    cnt.push(c);
                }
                stack.extend(cnt);
            } else {
                stack.push(c);
            }
        }
        String::from_iter(stack)
    }
}
