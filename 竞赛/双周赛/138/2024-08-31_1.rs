struct Solution;

impl Solution {
    pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        let mut res = 0;
        res += (num1 / 1000).min(num2 / 1000).min(num3 / 1000) * 1000;
        res += (num1 / 100 % 10).min(num2 / 100 % 10).min(num3 / 100 % 10) * 100;
        res += (num1 / 10 % 10).min(num2 / 10 % 10).min(num3 / 10 % 10) * 10;
        res += (num1 % 10).min(num2 % 10).min(num3 % 10);
        res
    }
}
