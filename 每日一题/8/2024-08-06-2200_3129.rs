struct Solution;

impl Solution {
    /// 记忆化搜索
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        fn dfs(memo: &mut Vec<Vec<Vec<i64>>>, i:i32, j: i32, k: i32, limit: i32) -> i64 {
            if i < 0 || j < 0 {
                return 0;
            }
            if i == 0 {
                if k == 1 && j <= limit {
                    return 1;
                } else {
                    return 0;
                }
            }
            if j == 0 {
                if k == 0 && i <= limit {
                    return 1;
                } else {
                    return 0;
                }
            }
            if memo[i as usize][j as usize][k as usize] >= 0 {
                return memo[i as usize][j as usize][k as usize];
            }
            memo[i as usize][j as usize][k as usize] = if k == 0 {
                dfs(memo, i - 1, j, 0, limit) + dfs(memo, i - 1, j, 1, limit) - dfs(memo, i - limit - 1, j, 1, limit)
            } else {
                dfs(memo, i, j - 1, 0, limit) + dfs(memo, i, j - 1, 1, limit) - dfs(memo, i, j - limit - 1, 0, limit)
            };
            memo[i as usize][j as usize][k as usize] = (memo[i as usize][j as usize][k as usize] + 1_000_000_007) % 1_000_000_007;
            memo[i as usize][j as usize][k as usize]
        }
        let mut memo = vec![vec![vec![-1; 2]; one as usize + 1]; zero as usize + 1];
        ((dfs(&mut memo, zero, one, 0, limit) + dfs(&mut memo, zero, one, 1, limit)) % 1_000_000_007) as i32
    }

    /// 递推
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
