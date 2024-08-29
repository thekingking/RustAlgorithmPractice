struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BinaryHeap;

        let mut cnt = BinaryHeap::from_iter(nums.into_iter().map(|x| -x as i64));
        let mut res = 0;
        while let Some(x) = cnt.pop() {
            if -x >= k as i64 {
                break;
            }
            if let Some(y) = cnt.pop() {
                cnt.push(x * 2 + y);
                res += 1;
            } else {
                break;
            }
        }
        res
    }
}
