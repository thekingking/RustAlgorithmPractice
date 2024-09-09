struct Solution;

impl Solution {
    pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::new();
        let m = values.len();
        let n = values[0].len();
        for i in 0..m {
            heap.push((-values[i][n - 1], i, n - 1));
        }
        let mut num = 1;
        let mut res = 0;
        while let Some((val, i, j)) = heap.pop() {
            res += num * (-val) as i64;
            if j > 0 {
                heap.push((-values[i][j - 1], i, j - 1));
            }
            num += 1;
        }
        res
    }
}