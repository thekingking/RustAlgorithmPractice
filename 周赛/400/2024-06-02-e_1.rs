struct Solution;

impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut max = 0;
        let mut num = 0;
        for c in s.chars() {
            if c == 'E' {
                num += 1;
            } else {
                num -= 1;
            }
            if max < num {
                max = num;
            }
        }
        max
    }
}