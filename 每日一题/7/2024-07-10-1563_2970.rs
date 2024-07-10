struct Solution;

impl Solution {
    /// 暴力枚举
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            for j  in i..nums.len() {
                let mut pre = 0;
                let mut flag = false;
                for k in 0..nums.len() {
                    if i <= k && k <= j {
                        continue;
                    }
                    if nums[k] <= pre {
                        flag = true;
                    }
                    pre = nums[k]
                }
                if !flag {
                    ans += 1;
                }
            }
        }
        ans
    }

    /// 不定长滑动窗口
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut i = 0;
        while i < n - 1 && nums[i] < nums[i + 1] {
            i += 1;
        }
        if i == n - 1 {
            return (n * (n + 1)) as i32 / 2;
        }
        let mut i = i as i32;
        let mut ans = i + 2;
        let mut j = n - 1;
        while j == n - 1 || nums[j] < nums[j + 1] {
            while i >= 0 && nums[i as usize] < nums[j] {
                i -= 1;
            }
            ans += i + 2;
            j -= 1;
        }
        ans
    }
}