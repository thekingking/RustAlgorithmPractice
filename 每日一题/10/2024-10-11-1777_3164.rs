struct Solution;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        use std::collections::HashMap;

        let mut cnt1 = HashMap::new();
        let mut cnt2 = HashMap::new();
        let mut max = 0;
        let mut res = 0;
        for x in nums1 {
            *cnt1.entry(x).or_insert(0) += 1;
            max = max.max(x);
        }
        for x in nums2 {
            *cnt2.entry(x).or_insert(0) += 1;
        }
        for (x, v) in cnt2 {
            let mut i = 1;
            while i * x * k <= max {
                if let Some(&j) = cnt1.get(&(i * x * k)) {
                    res += v * j;
                }
                i += 1;
            } 
        }
        res
    }
}