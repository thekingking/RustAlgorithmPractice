struct Solution;

impl Solution {
    pub fn maximum_total_sum(mut maximum_height: Vec<i32>) -> i64 {
        maximum_height.sort_unstable();
        let mut res = 0;
        let n = maximum_height.len();
        let mut pre = maximum_height[n - 1] as i64 + 1;
        for i in (0..n).rev() {
            if maximum_height[i] <= i as i32 {
                return -1;
            }
            if maximum_height[i] as i64 >= pre {
                pre -= 1;
                res += pre;
            } else {
                res += maximum_height[i] as i64;
                pre = maximum_height[i] as i64;
            }
        }
        res
    }
}