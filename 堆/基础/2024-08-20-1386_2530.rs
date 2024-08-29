struct Solution;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::BinaryHeap;

        let mut cnt = BinaryHeap::from(nums);
        let mut res = 0;
        for _ in 0..k {
            if let Some(x) = cnt.pop() {
                res += x as i64;
                cnt.push(x / 3 + if x % 3 != 0 { 1 } else { 0 });
            }
        }
        res
    }
}
