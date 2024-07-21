
struct Solution;

impl Solution {
    /// 题解
    pub fn minimum_length(s: String) -> i32 {
        let mut cnt = [0; 26];
        for &c in s.as_bytes() {
            cnt[(c - b'a') as usize] += 1;
        }
        let mut res = 0;
        for x in cnt {
            res += (x - 1) % 2 + 1;
        }
        res
    }

    /// 周赛
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
