struct Solution;

impl Solution {
    pub fn find_maximum_number(mut k: i64, x: i32) -> i64 {
        let mut num = 0;
        let mut stack = Vec::new();
        let mut dp = vec![0; 50];
        for i in 1..50 {
            dp[i] = dp[i - 1] * 2 + (1 << (i - 1));
        }
        for i in (1..50).rev() {
            let integer = (i - 1) / x;
            let remainder = i % x;
            let n = 2_i64.pow((i - integer - 1) as u32) * (dp[integer as usize] + (1 << integer) * num);
            if remainder == 0 {
                if n + num + 1 <= k {
                    stack.push(1);
                    k -= n;
                    if i % x == 0 {
                        num += 1;
                    }
                } else {
                    stack.push(0);
                }
            } else {
                if n + num <= k {
                    stack.push(1);
                    k -= n;
                    if i % x == 0 {
                        num += 1;
                    }
                } else {
                    stack.push(0);
                }
            }      
        }
        let mut res: i64 = 0;
        for i in 0..stack.len() {
            res += stack[i] << (stack.len() - i - 1);
        }
        res
    }
}