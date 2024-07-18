struct Solution;

impl Solution {
    /// dijkstra + binaryInsert
    pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        let mut cnt = vec![vec![]; n];
        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            let w = edge[2];
            cnt[x].push((y, w));
            cnt[y].push((x, w));
        }
        let mut vis = vec![false; n];
        let mut res = vec![-1; n];
        let mut queue = vec![(0, 0)];
        while let Some((i, x)) = queue.pop() {
            if !vis[i] {
                res[i] = x;
                for &(j, y) in &cnt[i] {
                    let dis = x + y;
                    if vis[j] || dis >= disappear[j] {
                        continue;
                    }
                    let mut l = 0;
                    let mut r = queue.len() as i32 - 1;
                    while l < r {
                        let mid = (l + r) / 2;
                        if  queue[mid as usize].1 >= dis {
                            l = mid + 1;
                        } else {
                            r = mid - 1;
                        }
                    }
                    queue.insert(l as usize, (j, dis));
                }
                vis[i] = true;
            }
        }
        res
    }

    pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        let mut cnt = vec![vec![]; n];
        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            let w = edge[2];
            cnt[x].push((y, -w));
            cnt[y].push((x, -w));
        }
        let mut res = vec![-1; n];
        let mut h = std::collections::BinaryHeap::new();
        h.push((0, 0));
        while let Some((x, i)) = h.pop() {
            if res[i] == -1 {
                res[i] = -x;
                for &(j, y) in &cnt[i] {
                    let dis = x + y;
                    if res[j] != -1 || -dis >= disappear[j] {
                        continue;
                    }
                    h.push((dis, j));
                }
            }
        }
        res
    }
}
