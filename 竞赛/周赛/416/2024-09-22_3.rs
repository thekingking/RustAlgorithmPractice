struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut cnt = vec![0; 26];
        let mut b = 0;
        for &w in word2.as_bytes() {
            cnt[(w - b'a') as usize] -= 1;
            b |= 1 << (w - b'a');
        }
        let mut l = 0;
        let mut r = 0;
        let bs = word1.as_bytes();
        let mut res = 0;
        while r < bs.len() || l < bs.len() {
            while r < bs.len() && b != 0 {
                cnt[(bs[r] - b'a') as usize] += 1;
                if cnt[(bs[r] - b'a') as usize] == 0 {
                    b ^= 1 << (bs[r] - b'a');
                }
                r += 1;
            }
            if b == 0 {
                res += (bs.len() - r + 1) as i64;
            }
            if cnt[(bs[l] - b'a') as usize] == 0 {
                b |= 1 << (bs[l] - b'a');
            }
            cnt[(bs[l] - b'a') as usize] -= 1;
            l += 1;
        }
        res
    }
}