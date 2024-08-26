struct Solution;

impl Solution {
    /// 函数栈
    pub fn calculate(s: String) -> i32 {
        fn dfs(bs: &Vec<u8>, idx: &mut usize) -> i32 {
            let mut stack = Vec::new();
            let mut num = 0;
            let mut flag = 1;
            while *idx < bs.len() {
                let c = bs[*idx];
                if c >= b'0' && c <= b'9' {
                    num = num * 10 + (c - b'0') as i32;
                } else {
                    stack.push(flag * num);
                    num = 0;
                    if c == b'+' {
                        flag = 1;
                    } else if c == b'-' {
                        flag = -1;
                    } else if c == b'(' {
                        *idx += 1;
                        num = dfs(bs, idx);
                    } else if c == b')' {
                        break;
                    }
                }
                *idx += 1;
            }
            stack.push(flag * num);
            stack.into_iter().sum()
        }
        let bs = s.into_bytes();
        let mut idx = 0;
        dfs(&bs, &mut idx)
    }

    /// 符号栈
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