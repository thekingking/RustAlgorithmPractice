struct Solution;

impl Solution {
    /// 线性DP，斐波那契数列
    pub fn count_house_placements(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut f1: i64 = 1;
        let mut f2: i64 = 1;
        for _ in 0..n {
            let f3 = (f2 + f1) % MOD;
            f1 = f2;
            f2 = f3;
        }
        (f2 * f2 % MOD) as i32
    }
}