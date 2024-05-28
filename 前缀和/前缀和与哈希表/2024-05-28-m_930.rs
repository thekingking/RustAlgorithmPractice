use std::collections::HashMap;
struct Solution;

impl Solution {
    // 哈希表
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut cnt = HashMap::new();
        let mut ans = 0;
        let mut sum = 0;
        for i in 0..nums.len() {
            cnt.entry(sum).and_modify(|x| *x += 1).or_insert(1);
            sum += nums[i];
            ans += cnt.get(&(sum - goal)).unwrap_or(&0);
        }
        ans
    }

    // 滑动窗口
    // 滑动窗口右边一步一步移动，左边按范围条件移动，由后向前寻找
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let (mut left1, mut left2, mut right) = (0, 0, 0);
        let (mut sum1, mut sum2) = (0, 0);
        let mut ret = 0;
        while right < nums.len() {
            sum1 += nums[right];
            while left1 <= right && sum1 > goal {
                sum1 -= nums[left1];
                left1 += 1;
            }
            sum2 += nums[right];
            while left2 <= right && sum2 >= goal {
                sum2 -= nums[left2];
                left2 += 1;
            }
            ret += left2 - left1;
            right += 1;
        }
        ret as i32
    }
}