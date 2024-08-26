struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for s in tokens {
            if let Ok(x) = s.parse::<i32>() {
                stack.push(x);
            } else {
                let x = stack.pop().unwrap();
                let c = s.as_bytes()[0];
                match c {
                    b'+' => *stack.last_mut().unwrap() += x,
                    b'-' => *stack.last_mut().unwrap() -= x,
                    b'*' => *stack.last_mut().unwrap() *= x,
                    b'/' => *stack.last_mut().unwrap() /= x,
                    _ => (),
                }
            }
        }
        stack.pop().unwrap()
    }
}