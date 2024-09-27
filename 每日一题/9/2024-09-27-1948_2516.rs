struct Solution;

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let mut cnt = vec![0; 3];
        let bs = s.as_bytes();
        for &c in bs {
            cnt[(c - b'a') as usize] += 1;
        }
        if cnt[0] < k || cnt[1] < k || cnt[2] < k {
            return -1;
        }
        let mut l = 0;
        let mut r = 0;
        let mut max = 0;
        while l < bs.len() && r < bs.len() {
            while r < bs.len() && cnt[(bs[r] - b'a') as usize] > k {
                cnt[(bs[r] - b'a') as usize] -= 1;
                r += 1;
            }
            max = max.max(r - l);
            cnt[(bs[l] - b'a') as usize] += 1;
            l += 1;
        }
        (bs.len() - max) as i32
    }
}