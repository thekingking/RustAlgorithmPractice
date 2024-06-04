struct Solution;

impl Solution {
    /// DFS
    /// 无环图，无空节点，即有n-1条边，n个节点，将edges化为邻接表
    /// 以当前节点为根节点进行dfs，计算子树中符合条件的节点个数，排列组合计算总的可能数
    pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut graph = vec![Vec::new(); n];
        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            let cost = edge[2];
            graph[x].push((y, cost));
            graph[y].push((x, cost));
        }

        fn dfs(graph: &Vec<Vec<(usize, i32)>>, p: usize, root: usize, cur: i32, signal_speed: i32) -> i32 {
            let mut res = 0;
            if cur == 0 {
                res += 1;
            }
            
            for &(v, c) in &graph[p] {
                if v != root {
                    res += dfs(graph, v, p, (cur + c) % signal_speed, signal_speed);
                }
            }
            res
        }

        let mut ans = vec![0; n];
        for i in 0..n {
            let mut pre = 0;
            for &(v, c) in &graph[i] {
                let cnt = dfs(&graph, v, i, c % signal_speed, signal_speed);
                ans[i] += pre * cnt;
                pre += cnt;
            }
        }
        ans
    }
}