use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let k = ((n * (n + 1) / 2 + 1) / 2) as i64;

        let check = |upper: usize| -> bool {
            let mut cnt = 0i64;
            let mut l = 0;
            let mut freq = HashMap::new();
            for (r, &x) in nums.iter().enumerate() {
                *freq.entry(x).or_insert(0) += 1; // 移入右端点
                while freq.len() > upper { // 窗口内元素过多
                    let e = freq.entry(nums[l]).or_insert(0);
                    *e -= 1;
                    if *e == 0 {
                        freq.remove(&nums[l]); // 移出左端点
                    }
                    l += 1;
                }
                cnt += (r - l + 1) as i64; // 右端点固定为 r 时，有 r-l+1 个合法左端点
                if cnt >= k {
                    return true;
                }
            }
            false
        };

        let mut left = 0;
        let mut right = n;
        while left + 1 < right {
            let mid = (left + right) / 2;
            if check(mid) {
                right = mid;
            } else {
                left = mid;
            }
        }
        right as _
    }
}