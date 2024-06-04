use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut cnt = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            cnt.entry(v).and_modify(|x: &mut Vec<_>| x.push(i as i64)).or_insert(vec![i as i64]);
        }
        pub fn get_sum_absolute_differences(nums: &Vec<i64>) -> Vec<Vec<i64>> {
            let sum = nums.iter().sum::<i64>();
            let n = nums.len() as i64;
            let mut pre_sum = 0;
            let mut ans = vec![vec![0, 0]; nums.len()];
            for (i, x) in nums.iter().enumerate() {
                ans[i][0] = *x;
                ans[i][1] = sum - pre_sum * 2 + x * (2 * i as i64 - n);
                pre_sum += x;
            }
            ans
        }
        let mut ans = vec![0; nums.len()];
        for v in cnt.values() {
            let res = get_sum_absolute_differences(&v);
            for x in res {
                ans[x[0] as usize] = x[1];
            }
        }
        ans
    }
}