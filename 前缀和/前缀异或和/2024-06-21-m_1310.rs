struct Solution;

impl Solution {
    /// 前缀异或和
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut cnt = vec![0; arr.len() + 1];
        let mut num = 0;
        for (i, &x) in arr.iter().enumerate() {
            cnt[i] = num;
            num ^= x;
        }
        cnt[arr.len()] = num;
        let mut res = vec![0; queries.len()];
        for (i, q) in queries.iter().enumerate() {
            res[i] = cnt[q[0] as usize] ^ cnt[q[1] as usize + 1];
        }
        res
    }
}