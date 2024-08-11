struct Solution;

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        for c in commands {
            let b = c.as_bytes()[0];
            let mut m1 = 0;
            let mut m2 = 0;
            if b == b'U' {
                m1 -= 1;
            } else if b == b'D' {
                m1 += 1;
            } else if b == b'R' {
                m2 += 1;
            } else {
                m2 -= 1;
            }
            if i + m1 >= 0 && i + m1 < n && j + m2 >= 0 && j + m2 < n {
                i += m1;
                j += m2;
            } else {
                break;
            }
        }
        i * n + j
    }
}
