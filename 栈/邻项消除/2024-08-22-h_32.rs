struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = Vec::new();
        stack.push((')', -1));
        for (i, c) in s.chars().enumerate() {
            if let Some(&(pc, _)) = stack.last() {
                if pc == '(' && c == ')' {
                    stack.pop();
                } else {
                    stack.push((c, i as i32));
                }
            }
        }
        let mut res = 0;
        let mut num = s.len() as i32;
        while let Some((_, i)) = stack.pop() {
            if res < num - i {
                res = num - i - 1;
            }
            num = i;
        }
        res
    }
}