
fn main() {
    println!("hello, world");
}

struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut ops = Vec::new();
        ops.push(1);
        let mut sign = 1;
        let mut res = 0;
        let mut i = 0;
        let n = s.len();
        let bs = s.into_bytes();
        while i < n {
            match bs[i] {
                b' ' => i += 1,
                b'+' => {
                    sign = *ops.last().unwrap();
                    i += 1;
                },
                b'-' => {
                    sign = -*ops.last().unwrap();
                    i += 1;
                },
                b'(' => {
                    ops.push(sign);
                    i += 1;
                },
                b')' => {
                    ops.pop();
                    i += 1;
                },
                _ => {
                    let mut num = 0;
                    while i < n && bs[i] >= b'0' && bs[i] <= b'9' {
                        num = num * 10 + (bs[i] - b'0') as i32;
                        i += 1;
                    }
                    res += sign * num;
                }
            }
        }
        res
    }
}