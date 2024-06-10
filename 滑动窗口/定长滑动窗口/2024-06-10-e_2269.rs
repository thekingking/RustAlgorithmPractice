struct Solution;

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let mut ans = 0;
        let mut div1 = 1;
        let mut div2 = 1;
        let mut m = 0;
        while m < k {
            div1 *= 10;
            m += 1;
        }
        loop {
            let t = num % div1 / div2;
            if t != 0 && num % t == 0 {
                ans += 1;
            }
            if div1 > num {
                break;
            }
            div1 = div1.saturating_mul(10);
            div2 = div2.saturating_mul(10);
        }
        ans
    }
}