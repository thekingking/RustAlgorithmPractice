struct Solution;

impl Solution {
    /// 参数压缩，将i,j,_and压缩到一块
    pub fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        fn dfs(nums: &Vec<i32>, and_values: &Vec<i32>, memo: &mut HashMap<i64, i32>, n: usize, m: usize, i: usize, j: usize, mut and_: i32) -> i32 {
            if n - i < m - j {
                return i32::MAX / 2;
            }
            if j == m {
                return if i == n { 0 } else { i32::MAX / 2 };
            }
            and_ &= nums[i];
            let mask = (i as i64) << 36 | (j as i64) << 32 | and_ as i64;
            if memo.contains_key(&mask) {
                return *memo.get(&mask).unwrap();
            }
            let mut res = dfs(nums, and_values, memo, n, m, i + 1, j, and_);
            if and_ == and_values[j] {
                res = res.min(dfs(nums, and_values, memo, n, m, i + 1, j + 1, -1) + nums[i]);
            }
            memo.insert(mask, res);
            res
        }
        let n = nums.len();
        let m = and_values.len();
        let mut memo = HashMap::new();
        let ans = dfs(&nums, &and_values, &mut memo, n, m, 0, 0, -1);
        if ans < i32::MAX / 2 { ans } else { -1 }
    }
}
