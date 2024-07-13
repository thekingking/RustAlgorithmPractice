struct Solution;

impl Solution {
    /// 简单贪心
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let mut sum: i32 = apple.iter().sum();
        let mut capacity = capacity;
        capacity.sort_unstable();
        for (i, &c) in capacity.iter().enumerate().rev() {
            if sum <= c {
                return (capacity.len() - i) as i32
            }
            sum -= c;
        }
        capacity.len() as i32
    }
}