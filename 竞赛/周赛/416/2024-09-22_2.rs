struct Solution;

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let mut cnt = BinaryHeap::from_iter(worker_times.into_iter().map(|x| Reverse((x as i64, x as i64 * 2, x as i64, 2))));
        let mut res = 0;
        for _ in 0..mountain_height {
            if let Some(Reverse((t, a, x, i))) = cnt.pop() {
                res = t;
                cnt.push(Reverse((t + a, x * (i + 1), x, i + 1)));
            }
        }
        res
    }
}