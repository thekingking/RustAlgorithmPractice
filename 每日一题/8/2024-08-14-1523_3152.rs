struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut cnt = vec![0; nums.len() + 1];
        let mut num = 0;
        for i in 1..nums.len() {
            if (nums[i - 1] + nums[i]) % 2 == 0 {
                num += 1;
            }
            cnt[i] = num;
        }
        let mut res = vec![false; queries.len()];
        for (i, q) in queries.iter().enumerate() {
            if cnt[q[0] as usize] == cnt[q[1] as usize] {
                res[i] = true;
            }
        }
        res
    }
}
