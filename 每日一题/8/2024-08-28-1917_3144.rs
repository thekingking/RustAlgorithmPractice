struct Solution;

impl Solution {
    /// 递推DP
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let mut dp = vec![0; s.len() + 1];
        let bs = s.as_bytes();
        let n = bs.len();
        for i in 1..=n {
            let mut max = 0;
            let mut res = i as i32;
            let mut cnt = vec![0; 26];
            for j in (0..i).rev() {
                let k = (bs[j] - b'a') as usize;
                cnt[k] += 1;
                if cnt[k] > max {
                    max = cnt[k];
                }
                if cnt.iter().all(|&x| x == max || x == 0) {
                    res = res.min(1 + dp[j]);
                }
            }
            dp[i] = res;
        }
        dp[bs.len()]
    }
    
    /// 记忆化搜索
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        use std::collections::HashMap;
        fn dfs(bs: &Vec<u8>, memo: &mut HashMap<(usize, usize), i32>, s: usize, e: usize) -> i32 {
            if memo.contains_key(&(s, e)) {
                return *memo.get(&(s, e)).unwrap();
            }
            let mut cnt = vec![0; 26];
            let mut max = 0;
            let mut res = (e - s) as i32;
            for i in s..e {
                let j = (bs[i] - b'a') as usize;
                cnt[j] += 1;
                if cnt[j] > max {
                    max = cnt[j];
                }
                if cnt.iter().all(|&x| x == max || x == 0) {
                    res = res.min(1 + dfs(bs, memo, i + 1, e));
                }
            }
            memo.insert((s, e), res);
            res
        }
        let bs = s.into_bytes();
        let mut memo = HashMap::new();
        dfs(&bs, &mut memo, 0, bs.len())
    }
}