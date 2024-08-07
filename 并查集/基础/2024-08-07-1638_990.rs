struct Solution;

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut graph = vec![vec![0; 26]; 26];
        let mut ne = Vec::new();
        for i in 0..26 {
            graph[i][i] = 1;
        }
        for e in equations {
            let bs = e.into_bytes();
            let a = (bs[0] - b'a') as usize;
            let b = (bs[3] - b'a') as usize;
            if bs[1] == b'=' {
                graph[a][b] = 1;
                graph[b][a] = 1;
            } else {
                ne.push((a, b))
            }
        }
        let mut cnt = vec![-1; 26];
        fn dfs(graph: &Vec<Vec<i32>>, cnt: &mut Vec<i32>, pre: usize, num: i32) {
            for i in 0..26 {
                if cnt[i] != -1 {
                    continue;
                }
                if graph[pre][i] == 1 {
                    cnt[i] = num;
                    dfs(graph, cnt, i, num);
                }
            }
        }
        for i in 0..26 {
            if cnt[i] != -1 {
                continue;
            }
            dfs(&graph, &mut cnt, i, i as i32);
        }
        for (a, b) in ne {
            if cnt[a] == cnt[b] {
                return false;
            }
        }
        true
    }
}
