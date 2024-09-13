struct Solution;

impl Solution {
  pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
    use std::collections::VecDeque;
        let mut ans = 0;
        let mut left = 0;
        let mut sum = 0i64;
        let mut q = VecDeque::new();
        for right in 0..charge_times.len() {
            // 1. 入
            while !q.is_empty() && charge_times[right] >= charge_times[*q.back().unwrap()] {
                q.pop_back();
            }
            q.push_back(right);
            sum += running_costs[right] as i64;

            // 2. 出
            while !q.is_empty() && charge_times[*q.front().unwrap()] as i64 + (right - left + 1) as i64 * sum > budget {
                if *q.front().unwrap() == left {
                    q.pop_front();
                }
                sum -= running_costs[left] as i64;
                left += 1;
            }

            // 3. 更新答案
            ans = ans.max(right - left + 1);
        }
        ans as _
  }
}