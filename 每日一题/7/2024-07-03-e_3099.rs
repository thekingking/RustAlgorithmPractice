struct Solution;

impl Solution {
    /// 简单题
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut n = x;
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        if x % sum == 0 {
            sum
        } else {
            -1
        }
    }
}