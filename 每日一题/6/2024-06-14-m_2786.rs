struct Solution;

impl Solution {
    /// 动态规划
    pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
        let x = x as i64;
        let mut even = if nums[0] % 2 == 0 { nums[0] as i64 } else { i32::MIN as i64 };
        let mut odd = if nums[0] % 2 == 1 { nums[0] as i64 } else { i32::MIN as i64};
        for &n in &nums[1..] {
            if n % 2 == 1 {
                odd = odd.max(even - x) + n as i64;
            } else {
                even = even.max(odd - x) + n as i64;
            }
        }
        odd.max(even)
    }
}
