struct Solution;

impl Solution {
    pub fn min_changes(mut n: i32, mut k: i32) -> i32 {
        let mut res = 0;
        while n != 0 && k != 0 {
            if n % 2 == 1 && k % 2 == 0 {
                res += 1;
            } else if n % 2 == 0 && k % 2 == 1 {
                return -1;
            }
            n /= 2;
            k /= 2;
        }
        res
    }
}
