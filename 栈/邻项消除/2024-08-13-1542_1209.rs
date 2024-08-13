struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut res = Vec::new();
        let mut stack = Vec::new();
        for &c in s.as_bytes() {
            if let Some(&pre) = res.last() {
                if pre == c {
                    let n = stack.len();
                    stack[n - 1] += 1;
                } else {
                    stack.push(1);
                }
            } else {
                stack.push(1);
            }
            res.push(c);
            if let Some(&last) = stack.last() {
                if last == k {
                    for _ in 0..k {
                        res.pop();
                    }
                    stack.pop();
                }
            }
        }
        unsafe {String::from_utf8_unchecked(res)}
    }
}
