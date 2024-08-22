struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut cnt = 0;
        let mut res = 0;
        let mut i = 0;
        let bs = s.as_bytes();
        while i < bs.len() {
            if bs[i] == b'(' {
                cnt += 1;
                res += 2;
            } else {
                if cnt > 0 {
                    if i + 1 < bs.len() && bs[i + 1] == b')' {
                        cnt -= 1;
                        res -= 2;
                        i += 1;
                    } else {
                        cnt -= 1;
                        res -= 1;
                    }
                } else {
                    if i + 1 < bs.len() && bs[i + 1] == b')' {
                        res += 1;
                        i += 1;
                    } else {
                        res += 2;
                    }
                }
            }
            i += 1;
        }
        res
    }
}