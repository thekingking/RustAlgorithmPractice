struct Solution;

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut res = 0;
        for i in 0..n / 2 {
            for j in 0..m / 2 {
                let a = grid[i][j];
                let b = grid[n - i - 1][j];
                let c = grid[i][m - j - 1];
                let d = grid[n - i - 1][m - j - 1];
                if a + b + c + d == 2 {
                    res += 2;
                } else if (a + b + c + d) % 2 == 1 {
                    res += 1;
                }
            }
        }
        if n % 2 == 1 && m % 2 == 1 {
            res += grid[n / 2][m / 2];
            let mut i = 0;
            let mut j = 0;
            let mut num = 0;
            let mut sum = 0;
            while i < n / 2 {
                let a = grid[i][m / 2];
                let b = grid[n - i - 1][m / 2];
                if a != b {
                    num += 1;
                }
                sum += a + b;
                i += 1;
            }
            while j < m / 2 {
                let a = grid[n / 2][j];
                let b = grid[n / 2][m - j - 1];
                if a != b {
                    num += 1;
                }
                sum += a + b;
                j += 1;
            }
            if num == 0 && sum % 4 != 0 {
                res += 2;
            } else if num != 0 {
                res += num;
            }
        } else if n % 2 == 1 && m % 2 == 0 {
            let mut i = 0;
            let mut sum = 0;
            let mut num = 0;
            while i < m / 2 {
                let a = grid[n / 2][i];
                let b = grid[n / 2][m - i - 1];
                if a != b {
                    num += 1;
                }
                sum += a + b;
                i += 1;
            }
            if num == 0 && sum % 4 != 0 {
                res += 2;
            } else if num != 0 {
                res += num;
            }
        } else if n % 2 == 0 && m % 2 == 1 {
            let mut i = 0;
            let mut sum = 0;
            let mut num = 0;
            while i < n / 2 {
                let a = grid[i][m / 2];
                let b = grid[n - i - 1][m / 2];
                if a != b {
                    num += 1;
                }
                sum += a + b;
                i += 1;
            }
            if num == 0 && sum % 4 != 0 {
                res += 2;
            } else if num != 0 {
                res += num;
            }
        }
        res
    }
}