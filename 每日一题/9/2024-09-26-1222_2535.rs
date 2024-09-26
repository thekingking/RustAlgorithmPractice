struct Solution;

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for mut x in nums {
            res += x;
            while x > 0 {
                res -= x % 10;
                x /= 10;
            }
        }
        res
    }
}