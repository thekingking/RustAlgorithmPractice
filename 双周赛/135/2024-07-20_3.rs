struct Solution;

impl Solution {
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
