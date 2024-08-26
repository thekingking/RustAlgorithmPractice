struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        fn dfs(bs: &Vec<u8>, idx: &mut usize) -> i32 {
            let mut stack = Vec::new();
            let mut num = 0;
            let mut flag = 1;
            while *idx < bs.len() {
                let c = bs[*idx];
                if c >= b'0' && c <= b'9' {
                    num = num * 10 + (c - b'0') as i32;
                }  else if c != b' ' {
                    if flag == 1 {
                        stack.push(num);
                    } else if flag == 2 {
                        stack.push(-num);
                    } else if flag == 3 {
                        let pre = stack.pop().unwrap();
                        stack.push(pre * num);
                    } else {
                        let pre = stack.pop().unwrap();
                        stack.push(pre / num);
                    }
                    num = 0;
                    if c == b'+' {
                        flag = 1;
                    } else if c == b'-' {
                        flag = 2;
                    } else if c == b'*' {
                        flag = 3;
                    } else if c == b'/' {
                        flag = 4;
                    } else if c == b'(' {
                        *idx += 1;
                        num = dfs(bs, idx);
                    } else if c == b')' {
                        break;
                    }
                }
                *idx += 1;
            }
            if flag == 1 {
                stack.push(num);
            } else if flag == 2 {
                stack.push(-num);
            } else if flag == 3 {
                let pre = stack.pop().unwrap();
                stack.push(pre * num);
            } else {
                let pre = stack.pop().unwrap();
                stack.push(pre / num);
            }
            stack.into_iter().sum()
        }
        let bs = s.into_bytes();
        let mut idx = 0;
        dfs(&bs, &mut idx)
    }
}