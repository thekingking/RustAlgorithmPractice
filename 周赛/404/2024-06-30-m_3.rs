struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let nums: Vec<usize> = nums.into_iter().map(|x| (x % k) as usize).collect();
        let k = k as usize;
        let mut cnt = vec![vec![1; k as usize]; k];
        let mut t = vec![false; k as usize];
        for i in 0..nums.len() {
            for j in 0..i {
                if !t[nums[j]] {
                    cnt[nums[j]][nums[i]] = cnt[nums[i]][nums[j]] + 1;
                    t[nums[j]] = true;
                }
            }
            t.fill(false)
        }
        let mut max = 0;
        for row in cnt {
            for x in row {
                if x > max {
                    max = x;
                }
            }
        }
        max
    }
}