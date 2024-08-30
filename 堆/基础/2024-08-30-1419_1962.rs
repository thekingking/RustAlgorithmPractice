struct Solution;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, mut k: i32) -> i32 {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::from(piles);
        while let Some(x) = heap.pop() {
            heap.push((x + 1) / 2);
            k -= 1;
            if k <= 0 {
                break;
            }
        }
        heap.into_iter().sum()
    }
}