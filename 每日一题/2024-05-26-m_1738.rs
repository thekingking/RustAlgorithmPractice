struct Solution;

impl Solution {
    // 双百
    // 前缀和加排序
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut answer = Vec::new();
        for i in 0..matrix.len() {
            let mut pre = 0;
            for j in 0..matrix[i].len() {
                pre ^= matrix[i][j];
                if i == 0 {
                    answer.push(pre);
                } else {
                    answer.push(answer[(i - 1) * matrix[i].len() + j] ^ pre);
                }
            }
        }
        answer.sort_unstable();
        *answer.iter().nth(answer.len() - k as usize).unwrap()
    }
}