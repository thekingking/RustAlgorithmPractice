struct Solution;

impl Solution {
    /// 暴力搜索
    pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        let n = values.len();
        let mut g = vec![vec![]; n];
        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let t = e[2];
            g[x].push((y, t));
            g[y].push((x, t));
        }

        fn dfs(x: usize, sum_time: i32, sum_value: i32, vis: &mut Vec<bool>, g: &Vec<Vec<(usize, i32)>>, values: &Vec<i32>, max_time: i32) -> i32 {
            let mut ans = if x == 0 { sum_value } else { 0 };
            for &(y, t) in &g[x] {
                if (sum_time + t) > max_time {
                    continue;
                }
                if vis[y] {
                    ans = ans.max(dfs(y, sum_time + t, sum_value, vis, g, values, max_time));
                } else {
                    vis[y] = true;
                    ans = ans.max(dfs(y, sum_time + t, sum_value + values[y], vis, g, values, max_time));
                    vis[y] = false;
                }
            }
            ans
        }

        let mut vis = vec![false; n];
        vis[0] = true;
        dfs(0, 0, values[0], &mut vis, &g, &values, max_time)
    }

    /// 迪杰斯特拉算法，对节点搜索进行优化，修剪枝叶
    pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        let n = values.len();
        let mut g = vec![vec![]; n];
        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let t = e[2];
            g[x].push((y, t));
            g[y].push((x, t));
        }

        // 迪杰斯特拉算法，求单源最短路径
        // 在这里，最短路径即可表示为节点返回到起始点0的最小时间花费
        let mut dis = vec![i32::MAX; n];
        dis[0] = 0;
        let mut h = BinaryHeap::new();
        h.push((0, 0));
        while let Some((dx, x)) = h.pop() {
            if -dx > dis[x] {
                continue;
            }
            for &(y, t) in &g[x] {
                let new_dis = -dx + t;
                if new_dis < dis[y] {
                    dis[y] = new_dis;
                    h.push((-new_dis, y));
                }
            }
        }

        fn dfs(x: usize, sum_time: i32, sum_value: i32, vis: &mut Vec<bool>, g: &Vec<Vec<(usize, i32)>>, values: &Vec<i32>, max_time: i32, dis: &Vec<i32>) -> i32 {
            let mut ans = if x == 0 { sum_value } else { 0 };
            for &(y, t) in &g[x] {
                if (sum_time + t + dis[y]) > max_time {
                    continue;
                }
                if vis[y] {
                    ans = ans.max(dfs(y, sum_time + t, sum_value, vis, g, values, max_time, dis));
                } else {
                    vis[y] = true;
                    ans = ans.max(dfs(y, sum_time + t, sum_value + values[y], vis, g, values, max_time, dis));
                    vis[y] = false;
                }
            }
            ans
        }

        let mut vis = vec![false; n];
        vis[0] = true;
        dfs(0, 0, values[0], &mut vis, &g, &values, max_time, &dis)
    }
}