struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = vec![0];
        for c in s.chars() {
            if c == '(' {
                stack.push(0);
            } else {
                let x = stack.pop().unwrap();
                *stack.last_mut().unwrap() += std::cmp::max(x * 2, 1);
            }
        }
        stack.pop().unwrap()
    }
}
