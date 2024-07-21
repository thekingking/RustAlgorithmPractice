struct Solution;

impl Solution {
    /// 题解
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = vec![0; k as usize + 1];
        let mut cnt2 = vec![0; k as usize + 1];
        let n = nums.len();
        for i in 0..n / 2 {
            let (mut p, mut q) = (nums[i], nums[n - i - 1]);
            if p > q {
                (p, q) = (q, p);
            }
            cnt[(q - p) as usize] += 1;
            cnt2[std::cmp::max(q, k - p) as usize] += 1;
        }
        let mut ans = n as i32;
        // 范围之外，需要改动两次的数目
        let mut sum2 = 0;
        // 枚举所有可能
        for i in 0..=(k as usize) {
            ans = std::cmp::min(ans, n as i32 / 2 - cnt[i] + sum2);
            sum2 += cnt2[i];
        }
        ans
    }

    /// 周赛
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let len = nums.len();
        for i in 0..len / 2 {
            cnt.entry((nums[i] - nums[len - i - 1]).abs()).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut res = i32::MAX;
        
        while cnt.len() != 0 {
            let mut max = 0;
            let mut num = 0;
            for (&k, &v) in &cnt {
                if v > num {
                    max = k;
                    num = v;
                }
            }
            if max <= k / 2 {
                return res.min(len as i32 / 2 - num);
            } else {
                cnt.remove(&max);
                let mut n = 0;
                for i in 0..len / 2 {
                    if (nums[i] - nums[len - i - 1]).abs() == max {
                        continue;
                    }
                    if nums[i] + max <= k || nums[i] - max >= 0 || nums[len - i - 1] + max <= k || nums[len - i - 1] - max >= 0 {
                        n += 1;
                    } else {
                        n += 2;
                    }
                }
                res = res.min(n);
            }
        }

        res
    }
}
