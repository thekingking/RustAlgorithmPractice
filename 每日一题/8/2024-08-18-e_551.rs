struct Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut a = 0;
        let mut l = 0;
        for c in s.chars() {
            if c == 'L' {
                l += 1;
            } else {
                if c == 'A' {
                    a += 1;
                }
                l = 0;
            }
            if l >= 3 || a >= 2 {
                return false;
            }
        }
        true
    }
}
