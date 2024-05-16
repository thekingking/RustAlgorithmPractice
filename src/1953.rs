struct Solution;

impl Solution {
    // 2024.05.16 每日一题
    // 贪心法
    pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
        let max = *milestones.iter().max().unwrap() as i64;
        let sum:i64 = milestones.iter().fold(0, |acc, &x| acc + x as i64);
        if max * 2 - sum > 1 {
            (sum - max) * 2 + 1
        } else {
            sum
        }
    }
}