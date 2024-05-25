struct Solution;

impl Solution {
    // 简单前缀和，关键在i32会溢出
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        // 由于i32可能会溢出，所以使用i64（我已经试过了，i32会溢出）
        let mut pre_sum:Vec<i64> = vec![0; candies_count.len() + 1];
        for i in 0..candies_count.len() {
            pre_sum[i + 1] = candies_count[i] as i64 + pre_sum[i];
        }
        let mut ans = vec![true; queries.len()];
        for (i, q) in queries.iter().enumerate() {
            if (q[1] as i64 + 1) * (q[2] as i64) <= pre_sum[q[0] as usize] || q[1] as i64 >= pre_sum[q[0] as usize + 1] {
                ans[i] = false;
            }
        }
        ans
    }
}