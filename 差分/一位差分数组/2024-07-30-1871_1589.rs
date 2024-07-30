struct Solution;

impl Solution {
    /// 差分，贪心，排序
    pub fn max_sum_range_query(mut nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        let mut cnt = vec![0; nums.len() + 1];
        for q in requests {
            cnt[q[0] as usize] += 1;
            cnt[q[1] as usize + 1] -= 1;
        }
        let mut sum = 0;
        for i in 0..cnt.len() {
            sum += cnt[i];
            cnt[i] = sum;
        }
        nums.sort_unstable_by_key(|x| -x);
        cnt.sort_unstable_by_key(|x| -x);
        let mut res = 0;
        for i in 0..nums.len() {
            if cnt[i] == 0 {
                break;
            }
            res = (res + cnt[i] as i64 * nums[i] as i64) % 1_000_000_007
        }
        res as i32
    }
}
