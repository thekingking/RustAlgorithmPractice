struct Solution;

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::new();
        let mut res = Vec::new();
        let mut tasks: Vec<(i32, i32, i32)> = tasks.into_iter().enumerate().map(|(i, task)| (task[0], task[1], i as i32)).collect();
        tasks.sort_unstable();
        let mut index = 1;
        heap.push((-tasks[0].1, -tasks[0].2));
        let mut t = tasks[0].0;
        while let Some((q, i)) = heap.pop() {
            res.push(-i);
            t += -q;
            while index < tasks.len() && t >= tasks[index].0 {
                heap.push((-tasks[index].1, -tasks[index].2));
                index += 1;
            }
            if heap.len() == 0 && index < tasks.len() {
                heap.push((-tasks[index].1, -tasks[index].2));
                t = tasks[index].0;
                index += 1;
            }
        }
        res
    }
}