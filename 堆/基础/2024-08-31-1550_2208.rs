struct Solution;

impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;

        let nums: Vec<i64> = nums.into_iter().map(|x| (x as i64) << 20).collect();
        let sum = nums.iter().sum::<i64>();
        let mut heap = BinaryHeap::from(nums);
        let mut half = sum >> 1;
        let mut res = 0;
        while let Some(x) = heap.pop() {
            half -= x >> 1;
            heap.push(x >> 1);
            res += 1;
            if half <= 0 {
                break;
            }
        }
        res
    }
}