struct Solution;

impl Solution {
    // 和官解差不多，暴力枚举每种可能
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut max = -1;
        let mut div = 0;
        for i in 0..divisors.len() {
            let mut cur = 0;
            for j in 0..nums.len() {
                if nums[j] % divisors[i] == 0 {
                    cur += 1;
                }
            }
            if cur > max || cur == max && div > divisors[i] {
                div = divisors[i];
                max = cur;
            }
        }
        div
    }
}