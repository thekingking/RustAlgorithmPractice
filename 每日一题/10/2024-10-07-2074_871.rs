struct Solution;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, mut stations: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;

        let mut sum = start_fuel;
        let mut heap = BinaryHeap::new();
        let mut res = 0;
        let mut pre = 0;
        stations.push(vec![target, 0]);
        for s in stations {
            sum -= s[0] - pre;
            while sum < 0 {
                if let Some(v) = heap.pop() {
                    sum += v;
                    res += 1;
                } else {
                    return -1;
                }
            }
            heap.push(s[1]);
            pre = s[0];
        }
        res
    }
}