struct Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let mut left = 0;
        let mut cnt = 0;
        let bs = s.as_bytes();
        let ls = locked.as_bytes();
        let n = bs.len();
        for i in 0..n {
            if bs[i] == b'(' {
                if left > 0 && ls[i] == b'0' {
                    left -= 1;
                    cnt += 1;
                } else {
                    left += 1;
                }
            } else {
                if left > 0 {
                    left -= 1;
                    cnt += ((ls[i] - b'0' + 1) % 2) as i32;
                } else {
                    cnt += ((ls[i] - b'0' + 1) % 2) as i32;
                    if cnt == 0 {
                        return false;
                    }
                    left += 1;
                }
            }
        }
        left == 0
    }
}