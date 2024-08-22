struct Solution;

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut res = vec![0; seq.len()];
        let mut idx = 0;
        for c in seq.chars() {
            if c == '(' {
                res[idx as usize] = idx & 1;
            } else {
                res[idx as usize] = (idx + 1) & 1;
            }
            idx += 1;
        }
        res
    }
}