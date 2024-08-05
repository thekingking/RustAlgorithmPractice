struct Solution;

impl Solution {
    pub fn find_integers(mut n: i32) -> i32 {
        let mut dp = vec![0; 32];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..32 {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        let mut cnt = Vec::new();
        while n != 0 {
            cnt.push(n % 2);
            n /= 2;
        }
        let mut pre = 0;
        let mut res = 0;
        while let Some(x) = cnt.pop() {
            if x == 1 && pre == 0 {
                res += dp[cnt.len() + 1];
            } else if x == 1 && pre == 1 {
                res += dp[cnt.len() + 1];
                break;
            }
            if cnt.len() == 0 {
                res += 1;
            }
            pre = x;
        }
        res
    }
}
