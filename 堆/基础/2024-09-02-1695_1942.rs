struct Solution;

impl Solution {
    pub fn smallest_chair(mut times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        use std::collections::BinaryHeap;
        let arr = times[target_friend as usize][0];
        let mut cnt = vec![false; times.len()];
        times.sort_unstable();
        let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        for t in times {
            while let Some(&(x, idx)) = heap.peek() {
                if -x <= t[0] {
                    heap.pop();
                    cnt[idx] = false;
                } else {
                    break;
                }
            }
            let mut j = 0;
            while j < cnt.len() && cnt[j] {
                j += 1;
            }
            if t[0] == arr {
                return j as i32;
            } else {
                cnt[j] = true;
                heap.push((-t[1], j));
            }
        }
        0
    }
}