struct Solution;

impl Solution {
    /// 一次遍历，简单题
    pub fn temperature_trend(temperature_a: Vec<i32>, temperature_b: Vec<i32>) -> i32 {
        let len = temperature_a.len();
        let mut res = 0;
        let mut num = 0;
        for i in 1..len {
            if temperature_a[i] > temperature_a[i - 1] && temperature_b[i] > temperature_b[i - 1]
            || temperature_a[i] == temperature_a[i - 1] && temperature_b[i] == temperature_b[i - 1]
            || temperature_a[i] < temperature_a[i - 1] && temperature_b[i] < temperature_b[i - 1] {
                num += 1;
            } else {
                num = 0;
            }
            res = res.max(num);
        }
        res
    }
}