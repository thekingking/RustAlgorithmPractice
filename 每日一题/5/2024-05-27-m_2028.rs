struct Solution;

impl Solution {
    // 模拟构造，简单题
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let res = mean * (rolls.len() as i32 + n) - rolls.iter().sum::<i32>();
        if res < n || res > 6 * n {
            return vec![];
        }
        let mut ans = vec![res / n; (n - res % n) as usize];
        ans.resize(n as usize, res / n + 1);
        ans
    }
}