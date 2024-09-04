struct Solution;

impl Solution {
    pub fn count_ways(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut i = 0;
        let mut res = 0;
        if 0 < nums[0] {
            res += 1;
        }
        if nums.len() as i32 > *nums.last().unwrap() {
            res += 1;
        }
        while i < nums.len() - 1 {
            if i as i32 >= nums[i] && (i as i32) < nums[i + 1] {
                res += 1;
            }
            i += 1;
        }
        res
    }
}