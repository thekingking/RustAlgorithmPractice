struct Solution;

impl Solution {
    /// 前缀和 排序
    pub fn min_operations(mut nums: Vec<i32>, mut queries: Vec<i32>) -> Vec<i64> {
        nums.sort();
        let mut queries: Vec<(usize, i32)> = (0..).zip(queries).collect();
        queries.sort_by_key(|x| x.1);
        let mut sum = 0;
        let mut index = 0;
        while index < nums.len() && nums[index] < queries[0].1 {
            sum += (queries[0].1 - nums[index]) as i64;
            index += 1;
        }
        let mut i = index;
        while i < nums.len() {
            sum += (nums[i] - queries[0].1) as i64;
            i += 1;
        }
        let mut ans = vec![0; queries.len()];
        ans[queries[0].0] = sum;
        let mut j = 1;
        while j < queries.len() {
            while index < nums.len() && nums[index] < queries[j].1 {
                sum -= 2 * (nums[index] - queries[j - 1].1) as i64;
                index += 1;
            }
            sum += (2 * index as i64 - nums.len() as i64) * ((queries[j].1 - queries[j - 1].1) as i64);
            ans[queries[j].0] = sum;
            j += 1;
        }
        ans
    }

    /// 前缀和 + 排序 + 二分查找 （参考灵神的解题思路）
    pub fn min_operations(mut nums: Vec<i32>, mut queries: Vec<i32>) -> Vec<i64> {
        nums.sort();
        let mut cnt = vec![0; nums.len() + 1];
        let mut sum = 0;
        for (i, &x) in nums.iter().enumerate() {
            cnt[i] = sum;
            sum += x as i64;
        }
        cnt[nums.len()] = sum;
        let mut res = vec![0; queries.len()];
        for (i, &q) in queries.iter().enumerate() {
            let index = Self::find(&nums, q);
            res[i] = (q as i64 * index as i64 - cnt[index]) + (cnt[nums.len()] - cnt[index] - q as i64 * (nums.len() - index) as i64);
        }
        res
    }

    fn find(nums: &Vec<i32>, n: i32) -> usize {
        let mut left = 1;
        let mut right = nums.len();
        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid - 1] == n {
                return mid;
            } else if nums[mid - 1] > n {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        right
    }
}