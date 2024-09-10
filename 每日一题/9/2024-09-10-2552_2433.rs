struct Solution;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut cnt4 = 0;
        let mut cnt3 = vec![0; n];
        for l in 2..n {
            let mut cnt2 = 0;
            for j in 0..l {
                if nums[j] < nums[l] {
                    cnt4 += cnt3[j];
                    cnt2 += 1;
                } else {
                    cnt3[j] += cnt2;
                }
            }
        }
        return cnt4;
    }
}