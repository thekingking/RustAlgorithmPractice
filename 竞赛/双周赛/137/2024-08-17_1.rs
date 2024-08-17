struct Solution;

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut res = Vec::new();
        for i in 0..=n - k {
            let mut flag = true;
            for j in 1..k {
                if nums[i + j] != nums[i + j - 1] + 1 {
                    flag = false;
                }
            }
            if flag {
                res.push(nums[i + k - 1])
            } else {
                res.push(-1);
            }
        }
        res
    }   
}