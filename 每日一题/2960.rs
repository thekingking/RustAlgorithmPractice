struct Solution;

impl Solution {

    // 差分法
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        let mut num = 0;
        for val in battery_percentages {
            if val - num > 0 {
                num += 1;
            }
        }
        num
    }
}