struct neighborSum {
    grid: Vec<Vec<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl neighborSum {

    fn new(grid: Vec<Vec<i32>>) -> Self {
        Self {
            grid,
        }
    }
    
    fn adjacent_sum(&self, value: i32) -> i32 {
        let n = self.grid.len();
        for i in 0..n {
            for j in 0..n {
                if self.grid[i][j] == value {
                    let mut sum = 0;
                    if i > 0 {
                        sum += self.grid[i - 1][j];
                    }
                    if j > 0 {
                        sum += self.grid[i][j - 1];
                    }
                    if i < n - 1 {
                        sum += self.grid[i + 1][j];
                    }
                    if j < n - 1 {
                        sum += self.grid[i][j + 1];
                    }
                    return sum;
                }
            }
        }
        0
    }
    
    fn diagonal_sum(&self, value: i32) -> i32 {
        let n = self.grid.len();
        for i in 0..n {
            for j in 0..n {
                if self.grid[i][j] == value {
                    let mut sum = 0;
                    if i > 0 && j > 0 {
                        sum += self.grid[i - 1][j - 1];
                    }
                    if i > 0 && j < n - 1 {
                        sum += self.grid[i - 1][j + 1];
                    }
                    if i < n - 1 && j > 0 {
                        sum += self.grid[i + 1][j - 1];
                    }
                    if j < n - 1 && i < n - 1 {
                        sum += self.grid[i + 1][j + 1];
                    }
                    return sum;
                }
            }
        }
        0
    }
}

/**
 * Your neighborSum object will be instantiated and called as such:
 * let obj = neighborSum::new(grid);
 * let ret_1: i32 = obj.adjacent_sum(value);
 * let ret_2: i32 = obj.diagonal_sum(value);
 */