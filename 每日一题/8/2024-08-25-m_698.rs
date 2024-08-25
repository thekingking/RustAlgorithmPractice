struct Solution;

impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
        let all = nums.iter().sum::<i32>();
        if all % k != 0 {
            return false;
        }
        let per = all / k;
        nums.sort_unstable();
        if *nums.last().unwrap() > per {
            return false;
        }
        let n = nums.len();
        let mut memo = vec![true; 1 << n];

        fn dfs(nums: &Vec<i32>, memo: &mut Vec<bool>, per: i32, s: i32, p: i32) -> bool {
            if s == 0 {
                return true;
            }
            if !memo[s as usize] {
                return memo[s as usize];
            }
            memo[s as usize] = false;
            for i in 0..nums.len() {
                if nums[i] + p > per {
                    break;
                }
                if (s >> i) & 1 == 1 && dfs(nums, memo, per, s ^ (1 << i), (p + nums[i]) % per) {
                    return true;
                }
            }
            return false;
        }
        dfs(&nums, &mut memo, per, (1 << n) - 1, 0)
    }
}