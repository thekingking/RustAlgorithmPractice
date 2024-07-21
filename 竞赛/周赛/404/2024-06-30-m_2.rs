struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut res1 = 0;
        let mut res2 = 0;
        let mut res3 = 0;
        let mut res4 = 0;
        let mut f1 = true;
        let mut f2 = false;
        for &x in &nums {
            if x % 2 == 0 {
                res1 += 1;
            } else {
                res2 += 1;
            }
            if f1 && x % 2 == 1 || !f1 && x % 2 == 0 {
                res3 += 1;
                f1 = !f1;
            }
            if f2 && x % 2 == 1 || !f2 && x % 2 == 0 {
                res4 += 1;
                f2 = !f2;
            }
        }
        res1.max(res2.max(res3.max(res4)))
    }
}