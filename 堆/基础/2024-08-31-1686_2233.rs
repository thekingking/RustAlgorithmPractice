struct Solution;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, mut k: i32) -> i32 {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::from_iter(nums.into_iter().map(|x| -x as i64));
        while let Some(x) = heap.pop() {
            heap.push(x - 1);
            k -= 1;
            if k <= 0 {
                break;
            }
        }
        heap.into_iter().fold(1, |acc, x| acc * -x % 1_000_000_007) as i32
    }
}
