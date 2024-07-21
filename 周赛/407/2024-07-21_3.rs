struct Solution;

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut res = 0;
        let mut sum = 0;
        let bs = s.as_bytes();
        let mut i = 0;
        let mut flag = false;
        while i < bs.len() {
            if bs[i] == b'1' {
                if flag {
                    res += sum;
                    flag = false;
                }
                sum += 1;
            } else {
                flag = true;
            }
            i += 1;
        }
        if flag {
            res += sum;
        }
        res
    }
}
