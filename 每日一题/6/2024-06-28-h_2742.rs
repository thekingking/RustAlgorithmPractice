struct Solution;

impl Solution {
    /// 记忆化搜索
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        fn dfs(i: i32, j: i32, mem: &mut Vec<Vec<i32>>, time: &Vec<i32>, cost: &Vec<i32>) -> i32 {
            if i < j {
                return 0;
            }
            if i < 0 {
                return i32::MAX / 2;
            }
            if mem[i as usize][j as usize + time.len()] == -1 {
                mem[i as usize][j as usize + time.len()] = std::cmp::min(dfs(i - 1, j + time[i as usize], mem, time, cost) + cost[i as usize], dfs(i - 1, j - 1, mem, time, cost));
            }
            mem[i as usize][j as usize + time.len()]
        }
        let n = cost.len();
        let mut mem = vec![vec![-1; 2 * n + 1]; n];
        dfs(n as i32 - 1, 0, &mut mem, &time, &cost)
    }

    /// 记忆化搜索，0-1背包
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        fn dfs(i: i32, j: i32, mem: &mut Vec<Vec<i32>>, time: &Vec<i32>, cost: &Vec<i32>) -> i32 {
            if j <= 0 {
                return 0;
            }
            if i < 0 {
                return i32::MAX / 2;
            }
            if mem[i as usize][j as usize] == -1 {
                mem[i as usize][j as usize] = std::cmp::min(dfs(i - 1, j - time[i as usize] - 1, mem, time, cost) + cost[i as usize], dfs(i - 1, j, mem, time, cost));
            }
            mem[i as usize][j as usize]
        }
        let n = cost.len();
        let mut mem = vec![vec![-1; n + 1]; n];
        dfs(n as i32 - 1, n as i32, &mut mem, &time, &cost)
    }

    
}