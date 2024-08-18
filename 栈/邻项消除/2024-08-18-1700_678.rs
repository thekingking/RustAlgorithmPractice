struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut l = 0;
        let mut cl = 0;
        for c in s.chars() {
            if c == '(' {
                l += 1;
            } else if c == ')' {
                if l > 0 {
                    l -= 1;
                } else if cl > 0 {
                    cl -= 1;
                } else {
                    return false;
                }
            } else {
                cl += 1;
                if l > 0 {
                    l -= 1;
                    cl += 1;
                }
            }
        }
        l == 0
    }
}
