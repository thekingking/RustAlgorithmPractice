struct Solution;

impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut dif = vec![0; n + 1];
        let mut ans = 0;
        let mut sum = 0;
        for i in 0..n {
            sum += dif[i];
            if (nums[i] + sum) % 2 == 0 {
                if i + k as usize > n {
                    return -1;
                }
                ans += 1;
                sum += 1;
                dif[i + k as usize] -= 1;
            }
        }
        ans
    }
}
