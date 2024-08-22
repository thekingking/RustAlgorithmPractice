struct Solution;

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut res = 0;
        let mut bit = 0;
        let mut t = n as i64 - 1;
        let mut x = x;
        while t != 0 || x != 0 {
            if x & 1 == 1 {
                res |= 1 << bit;
            } else {
                res |= (t & 1) << bit;
                t /= 2;
            }
            x /= 2;
            bit += 1;
        }
        res
    }
}