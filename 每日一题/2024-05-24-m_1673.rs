struct Solution;

impl Solution {
    // 单调栈
    // 看官解写的，单调栈，一开始写的时间复杂度过不了
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let n = nums.len();
        for i in 0..n {
            while ans.len() > 0 && (n - i + ans.len()) as i32 > k && *ans.last().unwrap() > nums[i] {
                ans.pop();
            }
            ans.push(nums[i]);
        }
        ans.truncate(k as usize);
        ans
    }
}