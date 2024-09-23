struct Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut res = 0;
        for x in values {
            res = res.max(cnt + x - 1);
            cnt = x.max(cnt - 1);
        }
        res
    }
}