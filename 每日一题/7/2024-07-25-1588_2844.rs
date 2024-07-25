struct Solution;

impl Solution {
    pub fn minimum_operations(num: String) -> i32 {
        let mut res1 = 0;
        let mut res2 = 0;
        let bs = num.as_bytes();
        let n = bs.len();
        while res1 < n && bs[n - res1 - 1] != b'5' {
            res1 += 1;
        }
        res1 += 1;
        while res1 < n && bs[n - res1 - 1] != b'2' && bs[n - res1 - 1] != b'7' {
            res1 += 1;
        }
        if res1 == n {
            res1 += 1;
        }
        while res2 < n && bs[n - res2 - 1] != b'0' {
            res2 += 1;
        }
        res2 += 1;
        while res2 < n && bs[n - res2 - 1] != b'0' && bs[n - res2 - 1] != b'5' {
            res2 += 1;
        }
        n.min(res1 - 1).min(res2 - 1) as i32
    }
}
