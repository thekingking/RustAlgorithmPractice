struct Solution;

impl Solution {
    /// 前缀和，存储前缀和奇偶数的数量
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let (mut odd, mut even) = (0_i64, 0_i64);
        let mut ans = 0_i64;
        for x in arr {
            if x % 2 == 1 {
                ans += even + 1;
                (odd, even) = (even + 1, odd);
            } else {
                ans += odd;
                (odd, even) = (odd, even + 1);
            }
        }
        (ans % 1_000_000_007) as i32
    }
}