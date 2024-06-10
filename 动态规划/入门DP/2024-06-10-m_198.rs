struct Solution;

impl Solution {
    /// 记忆化搜索
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn dfs(nums: &Vec<i32>, cache: &mut Vec<i32>, i: i32) -> i32 {
            if i < 0 {
                return 0;
            }
            if cache[i as usize] == -1 {
                cache[i as usize] = std::cmp::max(dfs(nums, cache, i - 1), dfs(nums, cache, i - 2) + nums[i as usize]);
            }
            cache[i as usize]
        }
        let mut cache = vec![-1; nums.len()];
        dfs(&nums, &mut cache, nums.len() as i32 - 1)
    }

    /// dp，由记忆化搜索推导而出
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut f1 = 0;
        let mut f2 = 0;
        for x in nums {
            let f3 = std::cmp::max(f2, f1 + x);
            f1 = f2;
            f2 = f3;
        }
        f2
    }
}
