struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut stack = Vec::new();
        let mut res = Vec::new();
        let mut left = 0;
        let mut right = 0;
        for c in s.chars() {
            if c == '(' {
                left += 1;
            } else {
                right += 1;
            }   
            stack.push(c);
            if left == right {
                let n = stack.len();
                res.extend(stack[1..n - 1].to_vec());
                stack = Vec::new();
            }
        }
        String::from_iter(res)
    }
}
