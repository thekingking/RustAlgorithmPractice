
fn main() {
    println!("hello, world");
}

struct Solution;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let zero = zero as usize;
        let one = one as usize;
        let limit = limit as usize;
        let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![vec![0; 2]; one + 1]; zero + 1];
        for i in 1..=zero.min(limit) {
            dp[i][0][0] = 1;
        }
        for j in 1..=one.min(limit) {
            dp[0][j][1] = 1;
        }
        for i in 1..=zero {
            for j in 1..=one {
                dp[i][j][0] = (dp[i - 1][j][0] + dp[i - 1][j][1] + if i > limit { MOD - dp[i - limit - 1][j][1] } else { 0 }) % MOD;
                dp[i][j][1] = (dp[i][j - 1][0] + dp[i][j - 1][1] + if j > limit { MOD - dp[i][j - limit - 1][0] } else { 0 }) % MOD;
            }
        }
        ((dp[zero][one][0] + dp[zero][one][1]) % MOD) as i32
    }
}
