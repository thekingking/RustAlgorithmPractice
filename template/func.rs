struct Utils;

const MOD: i64 = 1_000_000_007;


impl Utils {
    /// k个m相乘，适用于k较大时简化计算
    fn pow(mut x: i64, mut n: i32) -> i64 {
        let mut res = 1;
        while n > 0 {
            if n % 2 == 1 {
                res = res * x % MOD;
            }
            x = x * x % MOD;
            n /= 2;
        }
        res
    }
}