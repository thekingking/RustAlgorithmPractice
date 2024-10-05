struct Solution;

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let mut left = 0;
        let mut right = 10_i64.pow(14);
        while left < right {
            let mid = (left + right) / 2;
            let mut sum = 0;
            for &t in &time {
                sum += mid / t as i64;
                if sum > total_trips as i64{
                    break;
                }
            }
            if sum >= total_trips as i64 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
