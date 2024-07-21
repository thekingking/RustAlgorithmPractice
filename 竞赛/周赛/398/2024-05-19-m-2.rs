struct Solution;

impl Solution {
    // 周赛第二题
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut ans = vec![0; nums.len()];
        let mut answer = vec![];
        if nums.len() == 1 {
            return vec![true; queries.len()];
        }
        for i in 1..nums.len() {
            if (nums[i] + nums[i - 1]) % 2 == 0 {
                ans[i] = ans[i - 1] + 1;
            } else {
                ans[i] = ans[i - 1];
            }
        }
        for q in queries {
            if ans[q[0] as usize] == ans[q[1] as usize] {
                answer.push(true);
            } else {
                answer.push(false);
            }
        }
        answer
    }
}