struct Solution;

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        use std::collections::BinaryHeap;

        let mut wait = BinaryHeap::from_iter(servers.into_iter().enumerate().map(|(i, x)| (-x, -(i as i32))));
        let mut run: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
        let mut t = 0;
        let mut i = 0;
        let n = tasks.len();
        let mut ans = vec![0; n];
        while i < n {
            while let Some(&(nt, x, j)) = run.peek() {
                if -nt <= t {
                    run.pop();
                    wait.push((x, j));
                } else {
                    break;
                }
            }
            if let Some((x, j)) = wait.pop() {
                ans[i] = -j;
                run.push((-(t + tasks[i]), x, j));
            } else if let Some((nt, x, j)) = run.pop() {
                t = -nt;
                ans[i] = -j;
                run.push((nt - tasks[i], x, j));
            }
            i += 1;
            if t < i as i32 {
                t = i as i32;
            }
        }
        ans
    }
}