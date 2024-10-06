struct Solution;

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut vis = vec![false; n];
        let mut cnt1 = vec![vec![]; n];
        let mut cnt2 = vec![vec![]; n];
        for row in invocations {
            cnt1[row[0] as usize].push(row[1] as usize);
            cnt2[row[1] as usize].push(row[0] as usize);
        }
        fn dfs(cnt1: &Vec<Vec<usize>>, vis: &mut Vec<bool>, cur: usize) {
            for &x in &cnt1[cur] {
                if vis[x] {
                    continue;
                }
                vis[x] = true;
                dfs(cnt1, vis, x);
            }
        }   
        vis[k as usize] = true;
        dfs(&cnt1, &mut vis, k as usize);
        let mut res = Vec::new();
        let mut flag = true;
        for i in 0..n {
            if vis[i] {
                for &x in &cnt2[i] {
                    flag &= vis[x];
                }
            }
            if !flag {
                break;
            }
        }
        for i in 0..n {
            if flag && vis[i] {
                continue;
            }
            res.push(i as i32);
        }
        res
    }
}