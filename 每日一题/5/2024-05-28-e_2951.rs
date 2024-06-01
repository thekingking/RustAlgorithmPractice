struct Solution;

impl Solution {
    // 简单题
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in 1..(mountain.len() - 1) {
            if mountain[i] > mountain[i - 1] && mountain[i] > mountain[i + 1] {
                ans.push(i as i32);
            }
        }
        ans
    }
}