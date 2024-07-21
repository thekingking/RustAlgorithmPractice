struct Solution;

impl Solution {
    pub fn value_after_k_seconds(n: i32, k: i32) -> i32 {
        let mut num = vec![1; n as usize];
        for _ in 0..k {
            for j in 1..(n as usize) {
                num[j] = (num[j - 1] + num[j]) % 1_000_000_007;
            }
        }
        num[n as usize - 1]
    }
}
