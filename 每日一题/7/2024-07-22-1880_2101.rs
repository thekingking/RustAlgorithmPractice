struct Solution;

impl Solution {
    /// 化圆为有向图，Bitset优化floyd
    pub fn maximum_detonation(mut bombs: Vec<Vec<i32>>) -> i32 {
        let n = bombs.len();
        let mut g = vec![0i128; n];
        for i in 0..n {
            for j in 0..n {
                let dis = ((bombs[i][0] - bombs[j][0]) as i64).pow(2) + ((bombs[i][1] - bombs[j][1]) as i64).pow(2);
                if dis <= (bombs[i][2] as i64).pow(2) {
                    g[i] |= 1 << j;
                }
            }
        }
        for k in 0..n {
            for i in 0..n {
                if g[i] >> k & 1 != 0 {
                    g[i] |= g[k];
                }
            }
        }
        let mut res = 0;
        for x in g {
            res = res.max(x.count_ones() as i32);
        }
        res
    }

    /// 化圆为有向图，枚举起点，dfs搜索路径
    pub fn maximum_detonation(mut bombs: Vec<Vec<i32>>) -> i32 {
        let n = bombs.len();
        let mut g = vec![vec![]; n];
        for i in 0..n {
            for j in 0..n {
                let dis = ((bombs[i][0] - bombs[j][0]) as i64).pow(2) + ((bombs[i][1] - bombs[j][1]) as i64).pow(2);
                if dis <= (bombs[i][2] as i64).pow(2) {
                    g[i].push(j);
                }
            }
        }
        fn dfs(g: &Vec<Vec<usize>>, vis: &mut Vec<bool>, x: usize) -> i32 {
            vis[x] = true;
            let mut cnt = 1;
            for &y in &g[x] {
                if !vis[y] {
                    cnt += dfs(g, vis, y);
                }
            }
            cnt
        }

        let mut ans = 0;
        for i in 0..n {
            let mut vis = vec![false; n];
            ans = std::cmp::max(ans, dfs(&g, &mut vis, i));
        }
        ans
    }

    /// 暴力枚举
    pub fn maximum_detonation(mut bombs: Vec<Vec<i32>>) -> i32 {
        let mut cnt = Vec::new();
        let n = bombs.len();
        let mut vis = vec![false; n];
        let mut res = 0;
        for i in 0..n {
            cnt.push(bombs[i].clone());
            vis[i] = true;
            let mut cur = 1;
            let mut pre = 0;
            while pre != cur {
                pre = cur;
                for j in 0..n {
                    if vis[j] {
                        continue;
                    }
                    for k in 0..cnt.len() {
                        let dis = ((cnt[k][0] - bombs[j][0]) as i64).pow(2) + ((cnt[k][1] - bombs[j][1]) as i64).pow(2);
                        if dis <= (cnt[k][2] as i64).pow(2) {
                            cnt.push(bombs[j].clone());
                            vis[j] = true;
                            cur += 1;
                            break;
                        }
                    }
                }
                
            }
            res = std::cmp::max(res, cnt.len() as i32);
            cnt.clear();
            vis.fill(false);
        }
        res
    }
}
