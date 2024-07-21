struct Solution;

impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        let mut sum = 0;
        for &c in s.as_bytes() {
            if c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u' {
                sum += 1;
            }
        }
        if sum == 0 {
            return false;
        } else {
            return true;
        }

    }
}
