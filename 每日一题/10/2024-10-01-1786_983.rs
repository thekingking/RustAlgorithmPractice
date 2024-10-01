struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut dp = vec![0; 366];
        let mut j = 0;
        for i in 1..366 {
            if j < days.len() && i as i32 == days[j] {
                dp[i] = dp[i - 1] + costs[0];
                dp[i] = std::cmp::min(dp[i], dp[i.saturating_sub(7)] + costs[1]);
                dp[i] = std::cmp::min(dp[i], dp[i.saturating_sub(30)] + costs[2]);
                j += 1;
            } else {
                dp[i] = dp[i - 1];
            }
        }
        dp[365]
    }
}
