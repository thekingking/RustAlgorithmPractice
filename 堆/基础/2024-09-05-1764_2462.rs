struct Solution;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        use std::collections::BinaryHeap;

        let mut h1 = BinaryHeap::new();
        let mut h2 = BinaryHeap::new();

        let mut l = 0;
        let mut r = costs.len() - 1;

        let mut res = 0;
        let mut num = 0;
        while num < k {
            while h1.len() < candidates as usize && l <= r {
                h1.push(-costs[l]);
                l += 1;
            }
            while h2.len() < candidates as usize && l <= r {
                h2.push(-costs[r]);
                r -= 1;
            }
            let lm = -h1.pop().unwrap_or(i32::MIN / 2);
            let rm = -h2.pop().unwrap_or(i32::MIN / 2);
            if lm <= rm {
                res += lm as i64;
                h2.push(-rm);
            } else {
                res += rm as i64;
                h1.push(-lm);
            }
            num += 1;
        }
        res
    }
}