struct Solution;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut cnt = vec![0; 26];
        for (i, &c) in s.as_bytes().iter().enumerate() {
            cnt[(c - b'a') as usize] = i as i32;
        }
        let mut res = 0;
        for (i, &c) in t.as_bytes().iter().enumerate() {
            res += (cnt[(c - b'a') as usize] - i as i32).abs();
        }
        res
    }
}