struct Solution;

impl Solution {
    /// 简单题
    /// 枚举
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        const MUTEX: [[bool; 10]; 10] = [
            [false, false, false, false, false, false, false, false, false, false],
            [false, true, true, true, true, true, true, true, true, true],
            [false, true, false, true, false, true, false, true, false, true],
            [false, true, true, false, true, true, false, true, true, false],
            [false, true, false, true, false, true, false, true, false, true],
            [false, true, true, true, true, false, true, true, true, true],
            [false, true, false, false, false, true, false, true, false, false],
            [false, true, true, true, true, true, true, false, true, true],
            [false, true, false, true, false, true, false, true, false, true],
            [false, true, true, false, true, true, false, true, true, false]
        ];
        let mut res = 0;
        let mut cnt = vec![0; 10];
        for x in nums {
            let mut x = x;
            for i in 0..10 {
                if MUTEX[i][x as usize % 10] {
                    res += cnt[i];
                }
            }
            while x >= 10 {
                x /= 10;
            }
            cnt[x as usize] += 1;
        }
        res
    }
}