struct Solution;

impl Solution {
    /// 简单贪心
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut costs = costs;
        costs.sort_unstable();
        let mut coins = coins;
        let mut res = 0;
        for c in costs {
            if coins < c {
                return res;
            }
            coins -= c;
            res += 1;
        }
        res
    }
}