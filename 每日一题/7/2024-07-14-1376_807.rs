struct Solution;

impl Solution {
    /// 贪心
    /// 双百
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut row = vec![0; n];
        let mut col = vec![0; n];
        for (i, r) in grid.iter().enumerate() {
            for (j, &x) in r.iter().enumerate() {
                row[i] = row[i].max(x);
                col[j] = col[j].max(x);
            }
        }
        let mut res = 0;
        for (i, r) in grid.iter().enumerate() {
            for (j, &x) in r.iter().enumerate() {
                res += std::cmp::min(row[i], col[j]) - x;
            }
        }
        res
    }
}