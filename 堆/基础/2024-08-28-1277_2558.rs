struct Solution;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::from_iter(gifts.into_iter().map(|x| x as i64));
        for _ in 0..k {
            if let Some(x) = heap.pop() {
                heap.push((x as f64).sqrt() as i64);
            }
        }
        heap.into_iter().sum()
    }
}