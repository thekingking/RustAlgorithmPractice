struct Solution;

impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        for i in 1..grid[0].len() {
            if grid[0][i] == grid[0][i - 1] {
                return false;
            }
        }
        for g in &grid[1..] {
            if &grid[0] != g {
                return false
            }
        }
        true
    }
}