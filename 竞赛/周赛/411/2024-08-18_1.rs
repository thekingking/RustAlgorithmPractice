struct Solution;

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut res = 0;
        let bs = s.into_bytes();
        let n = bs.len();
        for i in 0..n {
            let mut zero = 0;
            let mut one = 0;
            for j in i..n {
                if bs[j] == b'0' {
                    zero += 1;
                } else {
                    one += 1;
                }
                if zero <= k || one <= k {
                    res += 1;
                } else {
                    break;
                }
            }
        }
        res
    }
}
