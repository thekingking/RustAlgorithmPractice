struct Solution;

impl Solution {
    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        let s1 = word1.as_bytes();
        let s2 = word2.as_bytes();
        let n = s1.len();
        let m = s2.len();
        let mut suf = vec![0; n + 1];
        suf[n] = m as i32;
        let mut j = m as i32 - 1;
        for i in (0..n).rev() {
            if j >= 0 && s1[i] == s2[j as usize] {
                j -= 1;
            } 
            suf[i] = j + 1;
        }
        let mut ans = vec![];
        let mut changed = false;
        j = 0;
        for (i, &c) in s1.iter().enumerate() {
            if c == s2[j as usize] || !changed && suf[i + 1] <= j + 1 {
                if c != s2[j as usize] {
                    changed = true;
                }
                ans.push(i as i32);
                j += 1;
                if j as usize == m {
                    return ans;
                }
            }
        }
        vec![]
    }
}
