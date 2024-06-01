struct Solution;

impl Solution {
    /// O(n2)
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let mut ans = 0;
        for i in 0..=limit {
            for j in 0..=limit {
                if i + j > n {
                    break;
                }
                if n - i - j <= limit {
                    ans += 1;
                }
            }
        }
        ans
    }

    /// O(n)
    /// 官解
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let mut ans = 0;
        for i in 0..=std::cmp::min(n, limit) {
            if n - i > limit * 2 {
                continue;
            }
            ans += std::cmp::min(n - i, limit) - std::cmp::max(0, n - i - limit) + 1;
        }
        ans
    }

    /// O(1)
    /// 灵山的题解，容斥原理
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let c2 = |n| if n > 1 { n * (n - 1) / 2 } else { 0 };
        c2(n + 2) - 3 * c2(n - limit + 1) + 3 * c2(n - 2 * limit) - c2(n - 3 * limit - 1)
    }
}