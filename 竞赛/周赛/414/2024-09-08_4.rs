struct Solution;

impl Solution {
    pub fn max_moves(kx: i32, ky: i32, mut positions: Vec<Vec<i32>>) -> i32 {
        const DIRS: [(i32, i32); 8] = [(2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)];
        let n = positions.len();
        let mut dis = vec![vec![vec![-1; 50]; 50]; positions.len()];
        for (i, p) in positions.iter().enumerate() {
            let px = p[0];
            let py = p[1];
            dis[i][px as usize][py as usize] = 0;
            let mut step = 1;
            let mut q = vec![(px, py)];
            while q.len() > 0 {
                let tmp = q;
                q = Vec::new();
                for p in tmp {
                    for (dx, dy) in DIRS {
                        let (x, y) = (p.0 + dx, p.1 + dy);
                        if x >= 0 && x < 50 && y >= 0 && y < 50 && dis[i][x as usize][y as usize] == -1 {
                            dis[i][x as usize][y as usize] = step;
                            q.push((x, y));
                        }
                    }
                }
                step += 1;
            }
        }

        positions.push(vec![kx, ky]);
        let mut memo = vec![vec![-1; 1 << n]; n + 1];
        let u = (1 << n) - 1;
        fn dfs(positions: &Vec<Vec<i32>>, dis: &Vec<Vec<Vec<i32>>>, memo: &mut Vec<Vec<i32>>, i: i32, mask: i32, u: i32, n: usize) -> i32 {
            if mask == 0 {
                return 0;
            }
            let mut res = memo[i as usize][mask as usize];
            if res != -1 {
                return res;
            }
            let x = positions[i as usize][0];
            let y = positions[i as usize][1];
            if (u ^ mask).count_ones() % 2 == 0 {
                for j in 0..n {
                    if (mask >> j) & 1 > 0 {
                        res = std::cmp::max(res, dfs(positions, dis, memo, j as i32, mask ^ (1 << j), u, n) + dis[j][x as usize][y as usize]);
                    }
                }
            } else {
                res = i32::MAX;
                for j in 0..n {
                    if (mask >> j) & 1 > 0 {
                        res = std::cmp::min(res, dfs(positions, dis, memo, j as i32, mask ^ (1 << j), u, n) + dis[j][x as usize][y as usize]);
                    }
                }
            }
            memo[i as usize][mask as usize] = res;
            res
        }
        dfs(&positions, &dis, &mut memo, n as i32, u, u, n)
    }   
}