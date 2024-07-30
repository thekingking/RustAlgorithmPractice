struct Solution;

impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut cnt = vec![0; n + 1];
        for (i, &x) in nums.iter().enumerate() {
            let x = x as usize;
            if i >= x {
                cnt[0] += 1;
                cnt[i - x + 1] -= 1;
                cnt[i + 1] += 1;
                cnt[n] -= 1;

            } else {
                cnt[i + 1] += 1;
                cnt[n - x + i + 1] -= 1;
            }
        }
        let mut sum = 0;
        let mut res = 0;
        let mut max = 0;
        for (i, &c) in cnt.iter().enumerate() {
            sum += c;
            if sum > max {
                max = sum;
                res = i;
            }
        }
        res as i32
    }
}