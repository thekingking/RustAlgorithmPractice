struct Solution;

impl Solution {
    /// 动态规划 or 记忆化搜索
    /// 灵神题解，今天有事就先抄抄了
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let u = (1 << n) - 1;
        let mut f = vec![vec![0; n]; u];
        f[0].fill(1);
        for s in 1..u {
            for (i, &pre) in nums.iter().enumerate() {
                if s >> i & 1 != 0 {
                    continue;
                }
                for (j, &x) in nums.iter().enumerate() {
                    if s >> j & 1 != 0 && (pre % x == 0 || x % pre == 0) {
                        f[s][i] += f[s ^ (1 << j)][j] as i64;
                    }
                }
            }
        }
        let mut res = 0;
        for i in 0..n {
            res += f[u ^ (1 << i)][i];
        }
        (res % 1_000_000_007) as i32
    }
}