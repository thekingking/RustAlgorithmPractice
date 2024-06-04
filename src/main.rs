use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
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