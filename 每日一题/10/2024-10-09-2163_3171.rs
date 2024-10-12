struct Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = i32::MAX;
        let mut left = 0;
        let mut bottom = 0;
        let mut right_or = 0;
        for right in 0..nums.len() {
            right_or |= nums[right];
            while left <= right && (nums[left] | right_or) > k {
                ans = ans.min((nums[left] | right_or) - k);
                left += 1;
                if bottom < left {
                    // 重新构建一个栈
                    for i in (left..right).rev() {
                        nums[i] |= nums[i + 1];
                    }
                    bottom = right;
                    right_or = 0;
                }
            }
            if left <= right {
                ans = ans.min(k - (nums[left] | right_or));
            }
        }
        ans
    }
}