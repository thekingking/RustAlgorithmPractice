struct Solution;

impl Solution {
    /// 滑动窗口
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut num = 0;
        let mut sum = 0;
        let mut res = 0;
        let mut i = 0;
        while i < arr.len() {
            while i < arr.len() && num < k {
                sum += arr[i];
                i += 1;
                num += 1;
            }
            if sum >= k * threshold {
                res += 1;
            }
            sum -= arr[i - k as usize];
            num -= 1;
        }
        res
    }
}