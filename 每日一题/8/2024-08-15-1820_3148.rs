struct Solution;

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut cnt = vec![i32::MIN / 2; n + 1];
        let mut max = i32::MIN / 2;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                max = max.max(cnt[j] - grid[i][j]).max(cnt[j + 1] - grid[i][j]);
                cnt[j] = grid[i][j].max(cnt[j].max(cnt[j + 1]));
            }
        }
        max
    }
}
