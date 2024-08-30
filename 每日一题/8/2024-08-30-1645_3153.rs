struct Solution;

impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        let mut cnt = vec![vec![0; 10]; 9];
        let mut res = 0;
        for (i, &x) in nums.iter().enumerate() {
            let mut n = x;
            let mut j = 0;
            while n > 0 {
                let y = n % 10;
                res += i as i64 - cnt[j][y as usize];
                cnt[j][y as usize] += 1;
                j += 1;
                n /= 10;
            }
        }
        res
    }
}
