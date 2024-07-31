struct Solution;

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut cnt = vec![vec![0; n + 1]; n + 1];
        for q in queries {
            let x1 = q[0] as usize;
            let y1 = q[1] as usize;
            let x2 = q[2] as usize;
            let y2 = q[3] as usize;
            cnt[x1][y1] += 1;
            cnt[x1][y2 + 1] -= 1;
            cnt[x2 + 1][y1] -= 1;
            cnt[x2 + 1][y2 + 1] += 1;
        }
        let mut mat = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                if i == 0 {
                    if j == 0 {
                        mat[i][j] = cnt[i][j];
                    } else {
                        mat[i][j] = cnt[i][j] + mat[i][j - 1];
                    }
                } else {
                    if j == 0 {
                        mat[i][j] = cnt[i][j] + mat[i - 1][j];
                    } else {
                        mat[i][j] = cnt[i][j] + mat[i][j - 1] + mat[i - 1][j] - mat[i - 1][j - 1];
                    }
                }
            }
        }
        mat
    }
}
