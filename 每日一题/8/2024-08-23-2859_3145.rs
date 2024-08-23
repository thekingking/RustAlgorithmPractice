struct Solution;

impl Solution {
    pub fn find_products_of_elements(queries: Vec<Vec<i64>>) -> Vec<i32> {
        fn sum_e(mut k: i64) -> i64 {
            let mut res = 0;
            let mut cnt1 = 0;
            let mut sum_i = 0;
            let mut n: i64 = 0;
            for i in (0..=63 - (k + 1).leading_zeros() as i64).rev() {
                let c = (cnt1 << i) + (i << i >> 1);
                if c <= k {
                    k -= c;
                    res += (sum_i << i) + ((i * (i - 1) / 2) << i >> 1);
                    sum_i += i;
                    cnt1 += 1;
                    n |= 1 << i;
                }
            }
            while k > 0 {
                res += n.trailing_zeros() as i64;
                n &= n - 1;
                k -= 1;
            }
            res
        }
        fn pow(mut x: i64, mut n: i64, m: i64) -> i32 {
            let mut res = 1 % m;
            while n != 0 {
                if n % 2 == 1 {
                    res = res * x % m;
                }
                x = x * x % m;
                n /= 2;
            }
            res as i32
        }
        let mut ans = Vec::new();
        for q in queries {
            let er = sum_e(q[1] + 1);
            let el = sum_e(q[0]);
            ans.push(pow(2, er - el, q[2]));
        }
        ans
    }
}