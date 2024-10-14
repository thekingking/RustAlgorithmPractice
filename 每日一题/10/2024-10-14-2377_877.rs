struct Solution;

impl Solution {
    pub fn super_egg_drop(k: i32, mut n: i32) -> i32 {
        let k = k as usize;
        let mut f = vec![0; k + 1];
        for i in 1.. {
            for j in (1..=k).rev() {
                f[j] += f[j - 1] + 1;
            }
            if f[k] >= n {
                return i;
            }
        }
        0
    }
}