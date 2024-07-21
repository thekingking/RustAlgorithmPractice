struct Solution;

impl Solution {
    /// 题解，我们Rust也有自己的一行代码
    pub fn min_changes(n: i32, k: i32) -> i32 {
        if n & k == k { (n ^ k).count_ones() as i32 } else { -1 }
    }

    /// 周赛
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
