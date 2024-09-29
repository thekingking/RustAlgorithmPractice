struct Solution;

impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut n = k - 1;
        let mut num = 0;
        for x in operations {
            if n % 2 == 1 && x == 1 {
                num += 1;
            }
            n /= 2;
        }
        (b'a' + (num % 26) as u8) as _
    }
}