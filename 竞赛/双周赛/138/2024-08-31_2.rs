struct Solution;

impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        let bs = s.as_bytes();
        let n = bs.len();
        let k = k as usize;
        let mut res = Vec::new();
        for i in 0..n / k {
            let mut c = 0;
            for j in 0..k {
                c = (c + bs[i * k + j] - b'a') % 26;
            }
            res.push((c + b'a') as char);
        }
        String::from_iter(res)
    }
}
