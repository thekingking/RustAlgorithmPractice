fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
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