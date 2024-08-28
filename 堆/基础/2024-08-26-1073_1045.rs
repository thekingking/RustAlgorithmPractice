struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        let mut tree = BinaryHeap::from(stones);
        while let Some(y) = tree.pop() {
            if let Some(x) = tree.pop() {
                if y > x {
                    tree.push(y - x);
                }
            } else {
                return y;
            }
        }
        0
    }
}