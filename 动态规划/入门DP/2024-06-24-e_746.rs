struct Solution;

impl Solution {
    /// 动态规划
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut f0 = cost[0];
        let mut f1 = cost[1];
        for &c in &cost[2..] {
            let new_f = std::cmp::min(f0, f1) + c;
            f0 = f1;
            f1 = new_f;
        }
        std::cmp::min(f0, f1)
    }
}