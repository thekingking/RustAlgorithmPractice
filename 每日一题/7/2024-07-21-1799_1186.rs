struct Solution;

impl Solution {
    /// 最大子数组加强版，先找最大子数组解法（经典DP），然后构建删除一个数的最大子数组，选择最大的数为结果
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
