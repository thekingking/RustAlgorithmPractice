struct Solution;

impl Solution {
    /// 爬楼梯换皮
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let mut dp = vec![0_i64; high as usize + 1];
        dp[0] = 1;
        let mut res = 0;
        for i in 1..=high {
            if i - zero >= 0 {
                dp[i as usize] += dp[(i - zero) as usize] % 1_000_000_007;
            }
            if i - one >= 0 {
                dp[i as usize] += dp[(i - one) as usize] % 1_000_000_007;
            }
            if i >= low {
                res += dp[i as usize];
            }
        }
        (res % 1_000_000_007) as i32
    }
}