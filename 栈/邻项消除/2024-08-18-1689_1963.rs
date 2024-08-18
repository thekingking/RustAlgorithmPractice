struct Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut l = 0;
        for c in s.chars() {
            if c == '[' {
                l += 1;
            } else {
                l = std::cmp::max(l - 1, 0);
            }
        }
        (l + 1) / 2
    }
}
