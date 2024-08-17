struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack = Vec::new();
        let mut l = Vec::new();
        let mut num = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                l.push(i - num);
                stack.push(c);
            } else if c == ')' {
                if l.len() != 0 {
                    l.pop();
                    stack.push(c);
                } else {
                    num += 1;
                }
            } else {
                stack.push(c);
            }
        }
        while let Some(left) = l.pop() {
            stack.remove(left);
        }
        String::from_iter(stack)
    }
}
