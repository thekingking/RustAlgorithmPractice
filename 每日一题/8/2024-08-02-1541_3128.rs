struct Solution;

impl Solution {
    /// 枚举 + 乘法原理
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        let m = grid[0].len();
        let mut row = vec![0; n];
        let mut col = vec![0; m];
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                row[i] += grid[i][j];
                col[j] += grid[i][j];
            }
        }
        for i in 0..n {
            for j in 0..m {
                if row[i] > 1 && col[j] > 1 && grid[i][j] != 0 {
                    res += ((row[i] - 1) * (col[j] - 1)) as i64;
                }
            }
        }
        res
    }
}