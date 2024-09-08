struct Solution;

impl Solution {
    pub fn find_maximum_score(nums: Vec<i32>) -> i64 {
        let mut pre = nums[0] as i64;
        let mut pre_i = 0;
        let mut res = 0;
        for (i, &x) in nums.iter().enumerate() {
            if x as i64 > pre {
                res += (i - pre_i) as i64 * pre;
                pre_i = i;
                pre = x as i64;
            }
        }
        res += (nums.len() - pre_i - 1) as i64 * pre;
        res
    }
}