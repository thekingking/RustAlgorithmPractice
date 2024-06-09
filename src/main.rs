

fn main() {
    println!("hello, world");
    Solution::max_coins(vec![3,1,5,8]);
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        fn helper(nums: &Vec<i32>, dp: &mut Vec<Vec<i32>>, left: usize, right: usize) -> i32 {
            if left >= right - 1 {
                return 0;
            }
            if dp[left][right] == 0 {
                for i in (left + 1)..right {
                    dp[left][right] = dp[left][right].max(nums[left] * nums[i] * nums[right] + helper(nums, dp, left, i) + helper(nums, dp, i, right));
                }
            }
            dp[left][right]
        }
        nums.insert(0, 1);
        nums.push(1);
        let len = nums.len();
        let mut dp = vec![vec![0; len]; len];
        helper(&nums, &mut dp, 0, len - 1)
    }
}
