struct Solution;

impl Solution {
    pub fn max_possible_score(mut start: Vec<i32>, mut d: i32) -> i32 {
        fn check(start: &Vec<i32>, score: i32, d: i32) -> bool {
            let mut pre_x = i32::MIN;
            for &s in start {
                let x = pre_x + score;
                if x > s + d {
                    return false;
                }
                pre_x = std::cmp::max(x, s);
            }
            true
        }

        start.sort_unstable();

        let n = start.len();
        let mut left = 0;
        let mut right = (start[n - 1] + d - start[0]) / (n as i32 - 1) + 1;
        while left + 1 < right {
            let mid = ((left as i64 + right as i64) / 2) as i32;
            if check(&start, mid, d) {
                left = mid;
            } else {
                right = mid;
            }
        }
        left
    }
}