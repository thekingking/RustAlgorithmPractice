
struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut cnt = vec![0; 26];
        for &c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            if cnt[idx] == 2 {
                cnt[idx] -= 1;
            } else {
                cnt[idx] += 1;
            }
        }
        cnt.iter().sum()
    }
}
