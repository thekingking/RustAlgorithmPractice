struct Solution;

impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut flag = false;
        let mut res = 0;
        for x in nums {
            if flag && cnt == x {
                res += 1;
            } else {
                cnt = x;
                flag = !flag;
            }
        }
        res + flag as i32
    }
}
