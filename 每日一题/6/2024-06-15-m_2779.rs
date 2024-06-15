struct Solution;

impl Solution {
    /// 排序+滑动窗口
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut ans = 0;
        let mut left = 0;
        let mut right = 0;
        while right < nums.len() {
            while right < nums.len() && nums[right] - nums[left] <= 2 * k {
                right += 1;
            }
            ans = ans.max((right - left) as i32);
            left += 1;
        }
        ans
    }

    /// 差分数组
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let m = nums.iter().max().unwrap();
        let mut diff = vec![0; (m + 2) as usize];
        for x in nums.iter() {
            diff[0.max(x - k) as usize] += 1;
            diff[(m + 1).min(x + k + 1) as usize] -= 1;
        }
        let mut res = 0;
        let mut count = 0;
        for x in diff.iter() {
            count += x;
            res = res.max(count);
        }
        res
    }
}
