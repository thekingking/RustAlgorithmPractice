struct Solution;

impl Solution {
    pub fn max_good_number(nums: Vec<i32>) -> i32 {
        fn help(a: i32, b: i32, c: i32) -> i32 {
            let mut res = a;
            let cnt1 = 32 - b.leading_zeros();
            res = (res << cnt1) | b;
            let cnt2 = 32 - c.leading_zeros();
            res = (res << cnt2) | c;
            res
        }
        let a = nums[0];
        let b = nums[1];
        let c = nums[2];
        help(a, b, c)
            .max(help(b, a, c))
            .max(help(c, b, a))
            .max(help(c, a, b))
            .max(help(a, c, b))
            .max(help(b, c, a))
    }
}