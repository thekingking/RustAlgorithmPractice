struct Solution;

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut left = n - 1;
        let mut right = 0;
        let mut bottom = m - 1;
        let mut top = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    if i < bottom {
                        bottom = i;
                    }
                    if j < left {
                        left = j;
                    }
                    if i > top {
                        top = i;
                    }
                    if j > right {
                        right = j;
                    }
                }
            }
        }
        ((right - left + 1) * (top - bottom + 1)) as i32
    }
}