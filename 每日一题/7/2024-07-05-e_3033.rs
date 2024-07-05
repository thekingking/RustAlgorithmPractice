struct Solution;

impl Solution {
    /// 简单题
    pub fn modified_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        for i in 0..n {
            let mut max = -1;
            for j in 0..m {
                if matrix[j][i] > max {
                    max = matrix[j][i];
                }
            }
            for j in 0..m {
                if matrix[j][i] == -1 {
                    matrix[j][i] = max;
                }
            }
        }
        matrix
    }
}