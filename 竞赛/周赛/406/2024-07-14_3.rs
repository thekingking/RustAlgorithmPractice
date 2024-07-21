struct Solution;

impl Solution {
    pub fn minimum_cost(m: i32, n: i32, mut horizontal_cut: Vec<i32>, mut vertical_cut: Vec<i32>) -> i32 {
        horizontal_cut.sort_unstable_by_key(|&x| -x);
        vertical_cut.sort_unstable_by_key(|&x| -x);
        let mut i = 0;
        let mut j = 0;
        let mut cost = 0;
        while i < horizontal_cut.len() && j < vertical_cut.len() {
            if horizontal_cut[i] > vertical_cut[j] {
                cost += horizontal_cut[i] * (j + 1) as i32;
                i += 1;
            } else {
                cost += vertical_cut[j] * (i + 1) as i32;
                j += 1;
            }
        }
        while i < horizontal_cut.len() {
            cost += horizontal_cut[i] * n;
            i += 1;
        }
        while j < vertical_cut.len() {
            cost += vertical_cut[j] * m;
            j += 1;
        }
        cost
    }
}