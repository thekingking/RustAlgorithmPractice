struct Solution;

impl Solution {
    /// 前缀异或和，注意题意，可以重新排列子序列
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut pre = 0;
        let mut cnt = vec![0; s.len() + 1];
        let s_byte = s.as_bytes();
        for (i, &b) in s_byte.iter().enumerate() {
            cnt[i] = pre;
            pre ^= 1 << (b - b'a');
        }
        cnt[s_byte.len()] = pre;
        let mut ans = vec![false; queries.len()];
        for (i, q) in queries.iter().enumerate() {
            if q[2] * 2 >= q[1] - q[0] || q[2] >= 13 {
                ans[i] =true;
                continue;
            }
            let xor = cnt[q[0] as usize] ^ cnt[q[1] as usize + 1];
            let mut num = 0;
            for i in 0..26 {
                if xor & (1 << i) != 0 {
                    num += 1;
                }
            }
            if num / 2 <= q[2] {
                ans[i] = true;
            }
        }
        ans
    }
}