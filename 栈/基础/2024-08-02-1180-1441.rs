struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut res = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while i < n && j < target.len() {
            res.push("Push".to_string());
            if target[j] == i + 1 {
                j += 1;
            } else {
                res.push("Pop".to_string());
            }
            i += 1;
        }
        res
    }
}