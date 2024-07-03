struct Solution;

impl Solution {
    /// 贪心
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut cnt = vec![0; capacity.len()];
        for i in 0..capacity.len() {
            cnt[i] = capacity[i] - rocks[i];
        }
        cnt.sort_unstable();
        let mut additional_rocks = additional_rocks;
        for (i, &x) in cnt.iter().enumerate() {
            if x > additional_rocks {
                return i as i32;
            }
            additional_rocks -= x;
        }
        cnt.len() as i32
    }
}