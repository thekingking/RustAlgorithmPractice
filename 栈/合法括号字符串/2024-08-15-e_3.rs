struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '{' | '(' | '[' => stack.push(c),
                ')' => {
                    if !(stack.len() > 0 && stack.pop().unwrap() == '(') {
                        return false;
                    }
                },
                '}' => {
                    if !(stack.len() > 0 && stack.pop().unwrap() == '{') {
                        return false;
                    }
                },
                ']' => {
                    if !(stack.len() > 0 && stack.pop().unwrap() == '[') {
                        return false;
                    }
                },
                _ => ()
            }
        }
        stack.len() == 0
    }
}
