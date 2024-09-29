struct Solution;

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut res = 100;
        for mut x in nums {
            let mut sum = 0;
            while x > 0 {
                sum += x % 10;
                x /= 10;
            }
            res = res.min(sum);
        }
        res
    }
}
