struct Solution;

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let n = grid[0].len();
        let mut dp = vec![0; n];
        let mut index = n;
        let mut res = 0;
        for row in grid {
            let mut sum = 0;
            for (j, &c) in row.iter().enumerate() {
                match c {
                    'X' => sum += 1,
                    'Y' => sum -= 1,
                    _ => (),
                }
                if c == 'X' && j < index {
                    index = j;
                }
                dp[j] += sum;
                if dp[j] == 0 && j >= index {
                    res += 1;
                }
            }
        }
        res
    }
}