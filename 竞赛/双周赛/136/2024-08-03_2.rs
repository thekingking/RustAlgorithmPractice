struct Solution;

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let mut r = 0;
        let mut c = 0;
        let n = grid.len();
        let m = grid[0].len();
        for row in grid.iter() {
            let mut left = 0;
            let mut right = m - 1;
            while left < right {
                if row[left] != row[right] {
                    r += 1;
                }
                right -= 1;
                left += 1;
            }
        }
        for j in 0..m {
            let mut left = 0;
            let mut right = n - 1;
            while left < right {
                if grid[left][j] != grid[right][j] {
                    c += 1;
                }
                right -= 1;
                left += 1;
            }
        }
        std::cmp::min(r, c)
    }
}