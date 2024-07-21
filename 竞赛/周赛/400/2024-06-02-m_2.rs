struct Solution;

impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_by_key(|x| x[0]);
        let mut work = 0;
        let mut end = 0;
        for m in meetings {
            if m[0] > end + 1 {
                work += m[0] - end - 1;
            }
            end = end.max(m[1]);
        }
        work + days - end
    }
}