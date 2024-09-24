struct Solution;

impl Solution {
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let bs = text.as_bytes();
        let n = bs.len();
        let ps = pattern.as_bytes();
        let a = ps[0];
        let b = ps[1];
        let mut res = 0;
        if a == b {
            let mut num = 1;
            for i in 0..n {
                if bs[i] == a {
                    num += 1;
                }
            }
            for i in 1..num {
                res += i;
            }
        } else {
            let mut ra = 0;
            let mut rb = 0;
            let mut na = 1;
            let mut nb = 1;
            for i in 0..n {
                if bs[i] == b {
                    ra += na;
                } else if bs[i] == a {
                    na += 1;
                }
            }
            for i in (0..n).rev() {
                if bs[i] == a {
                    rb += nb;
                } else if bs[i] == b {
                    nb += 1;
                }
            }
            res = ra.max(rb);
        }
        res
    }
}