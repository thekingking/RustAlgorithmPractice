struct Solution;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut cnt = s.into_bytes();
        if x > y {
            cnt.reverse();
        }
        let (x, y) = (x.min(y), x.max(y));
        let mut stack = Vec::new();
        let mut res = 0;
        for c in cnt {
            if stack.len() > 0 && *stack.last().unwrap() == b'b' && c == b'a' {
                stack.pop();
                res += y;
            } else {
                stack.push(c);
            }
        }
        let cnt = stack;
        let mut stack = Vec::new();
        for c in cnt {
            if stack.len() > 0 && *stack.last().unwrap() == b'a' && c == b'b' {
                stack.pop();
                res += x;
            } else {
                stack.push(c);
            }
        }
        res
    }
}