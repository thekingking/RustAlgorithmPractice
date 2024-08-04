struct Solution;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut cnt = std::collections::HashMap::new();
        let mut res = n - 1;
        let mut ans = vec![0; queries.len()];
        let mut vis = vec![false; n as usize];
        for (i, q) in queries.iter().enumerate() {
            let u = q[0];
            let v = q[1];
            if !vis[u as usize] && !vis[v as usize] {
                if let Some(&t) = cnt.get(&u) {
                    if t < v {
                        cnt.insert(u, v);
                        for j in t..v {
                            if !vis[j as usize] {
                                vis[j as usize] = true;
                                res -= 1;
                            }

                        }
                    }
                } else {
                    cnt.insert(u, v);
                    for j in u + 1..v {
                        if !vis[j as usize] {
                            vis[j as usize] = true;
                            res -= 1;
                        }
                    }
                }
            }
            ans[i] = res;
        }
        ans
    }
}
