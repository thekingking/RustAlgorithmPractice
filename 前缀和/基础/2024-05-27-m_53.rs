struct Solution;

impl Solution {
    // 前缀和，也是妙到家了
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut pre = 0;
        let mut min_pre = 0;
        for n in nums {
            pre += n;
            ans = ans.max(pre - min_pre);
            min_pre = pre.min(min_pre);
        }
        ans
    }

    // 动态规划，妙到家了
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut pre = 0;
        let mut max = nums[0];
        for n in nums {
            pre = n.max(pre + n);
            max = max.max(pre);
        }
        max
    }
}