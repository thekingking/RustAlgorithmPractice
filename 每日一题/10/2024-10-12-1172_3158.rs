struct Solution;

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 51];
        let mut res = 0;
        for x in nums {
            cnt[x as usize] += 1;
        }
        for i in 1..51 {
            if cnt[i] == 2 {
                res ^= i as i32;
            }
        }
        res
    }
}