struct Solution;

impl Solution {
    /// 枚举 + 记忆化搜索
    pub fn sum_of_powers(mut nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        fn dfs(f: &mut HashMap<i64, i64>, nums: &Vec<i32>, i: i64, j: i64, n: i64, k: i64, mi: i64) -> i64 {
            if i >= n {
                return if k == 0 { mi as i64 } else { 0 };
            }
            if n - i < k {
                return 0;
            }
            let key: i64 = (mi<< 18) | (i << 12) | (j<< 6) | k;
            if f.contains_key(&key) {
                return *f.get(&key).unwrap();
            }
            let mut ans = dfs(f, nums, i + 1, j, n,  k, mi);
            if j == n {
                ans += dfs(f, nums, i + 1, i, n, k - 1, mi);
            } else {
                ans += dfs(f, nums, i + 1, i, n, k - 1, std::cmp::min(mi, (nums[i as usize] - nums[j as usize]) as i64));
            }
            ans %= 1_000_000_007;
            f.insert(key, ans);
            ans
        }
        
        nums.sort_unstable();
        let mut f: HashMap<i64, i64> = HashMap::new();
        let n = nums.len();

        dfs(&mut f, &nums, 0, n as i64, n as i64, k as i64, i32::MAX as i64) as i32
    }
}
