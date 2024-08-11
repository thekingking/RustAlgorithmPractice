struct Solution;

impl Solution {
    pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        fn dfs(cnt: &Vec<Vec<usize>>, pre: usize, cur: usize, res: &mut i32) -> i32 {
            if cur != 0 && cnt[cur].len() == 1 {
                *res += 1;
                return 1;
            }
            let mut sum = 1;
            let mut pre_num = 0;
            let mut flag = true;
            for &x in &cnt[cur] {
                if x == pre {
                    continue;
                }
                let n = dfs(cnt, cur, x, res);
                if pre_num != 0 && n != pre_num {
                    flag = false;
                }
                pre_num = n;
                sum += n;
            }
            if flag {
                *res += 1;
            }
            sum
        }
        let n = edges.len() + 1;
        let mut cnt = vec![vec![]; n];
        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            cnt[a].push(b);
            cnt[b].push(a);
        }
        let mut res = 0;
        dfs(&cnt, n, 0, &mut res);
        res
    }
}
