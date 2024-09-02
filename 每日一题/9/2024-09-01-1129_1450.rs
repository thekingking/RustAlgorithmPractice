struct Solution;

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let mut cnt = vec![0; 1002];
        let n = start_time.len();
        for i in 0..n {
            cnt[start_time[i] as usize] += 1;
            cnt[end_time[i] as usize + 1] -= 1;
        }
        let mut res = 0;
        for i in 0..=query_time {
            res += cnt[i as usize];
        }
        res
    }
}