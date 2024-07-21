struct Solution;

impl Solution {
    pub fn minimum_operations(mut nums: Vec<i32>, target: Vec<i32>) -> i64 {
        fn dfs(nums: &mut Vec<i32>, s: usize, e: usize) -> i64 {
            if s == e {
                return nums[s] as i64;
            }
            let mut min = i32::MAX;
            for &x in &nums[s..=e] {
                if min > x {
                    min = x;
                }
            }
            let mut res = min as i64;
            for i in s..=e {
                nums[i] -= min;
            }
            let mut l = s;
            let mut r = s;
            while r <= e && l <= e {
                while l <= e && nums[l] == 0 {
                    l += 1;
                }
                if l > e {
                    break;
                }
                r = l;
                while r <= e && nums[l] != 0 {
                    r += 1;
                }
                res += dfs(nums, l, r - 1);
                l = r;
            }
            res
        }

        for i in 0..nums.len() {
            nums[i] -= target[i];
        }
        let mut l = 0;
        let mut r = 0;
        let len = nums.len();
        let mut res = 0;
        while l < len && r < len {
            while l < len && nums[l] == 0 {
                l += 1;
            }
            if l == len {
                break;
            }
            r = l;
            if nums[l] > 0 {
                while r < len && nums[r] > 0 {
                    r += 1;
                }
            } else {
                while r < len && nums[r] < 0 {
                    nums[r] = - nums[r];
                    r += 1;
                }
            }
            res += dfs(&mut nums, l, r - 1);
            l = r;
        }
        res
    }
}
