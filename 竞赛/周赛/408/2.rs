struct Solution;

impl Solution {
    pub fn non_special_count(l: i32, r: i32) -> i32 {
        fn helper(n: i64) -> bool {
            let mut i = 2;
            while i * i <= n {
                if n % i == 0 {
                    return true;
                }
                i += 1;
            }
            false
        }

        let mut sum = 2;
        let mut res = 0;
        while sum * sum <= r as i64 {
            if sum * sum >= l as i64 {
                res += 1;
            }
            sum += 1;
            while helper(sum) {
                sum += 1;
            }
        }
        r - l - res + 1
    }
}