struct Solution;

impl Solution {
    /// 最长子数组加强版，先找最长子数组解法（经典DP），然后由最长子数组选择是否减去一个值（从大小判断）
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let mut res = arr[0];
        let mut dp0 = arr[0];
        let mut dp1 = 0;
        for &x in &arr[1..] {
            dp1 = std::cmp::max(dp1 + x, dp0);
            dp0 = std::cmp::max(dp0, 0) + x;
            res = res.max(dp0.max(dp1));
        }
        res
    }
}
