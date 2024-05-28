fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
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