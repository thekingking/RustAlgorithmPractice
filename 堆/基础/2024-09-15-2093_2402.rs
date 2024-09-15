struct Solution;

impl Solution {
  pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    use std::collections::BinaryHeap;
    let n = n as usize;
    let mut cnt = vec![0; n];
    let mut run = BinaryHeap::new();
    let mut wait: BinaryHeap<(i64, i32)> = BinaryHeap::new();
    let mut time = 0;
    meetings.sort_unstable();
    for i in 0..n {
      run.push(-(i as i32));
    }
    for i in 0..meetings.len() {
      time = time.max(meetings[i][0] as i64);
      while let Some(&(t, j)) = wait.peek() {
        if -t <= time || run.len() == 0 {
          time = time.max(-t);
          wait.pop();
          run.push(j);
        } else {
          break;
        }
      }
      if let Some(j) = run.pop() {
        cnt[(-j) as usize] += 1;
        wait.push((-(time + meetings[i][1] as i64 - meetings[i][0] as i64), j));
      }
    }
    let mut index = 0;
    for i in 0..n {
      if cnt[index as usize] < cnt[i] {
        index = i as i32;
      }
    }
    index
  }
}