struct Solution;

impl Solution {
    pub fn ways_to_reach_stair(k: i32) -> i32 {
        fn dfs(memo: &mut Vec<Vec<Vec<i32>>>, k:i32, i: i32, j: i32, pre: bool) -> i32 {
            let num = 2_i32.pow(j as u32) - i;
            let mut res = 0;
            if num > k + 1 || num < 0 {
                return 0;
            } else if num == k {
                res += 1;
            }  
            if memo[pre as usize][i as usize][j as usize] != -1 {
                return memo[pre as usize][i as usize][j as usize];
            }
            if pre {
                res += dfs(memo, k, i + 1, j, false);
            }
            res += dfs(memo, k, i, j + 1, true);
            memo[pre as usize][i as usize][j as usize] = res;
            res
        }
        let mut memo = vec![vec![vec![-1; 32]; 32]; 2];
        dfs(&mut memo, k, 0, 0, true)
    }
}