struct Solution;

impl Solution {
    /// 题解
    pub fn does_alice_win(s: String) -> bool {
        s.as_bytes().iter().any(|c| [b'a', b'e', b'i', b'o', b'u'].contains(c))
    }

    /// 周赛
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
