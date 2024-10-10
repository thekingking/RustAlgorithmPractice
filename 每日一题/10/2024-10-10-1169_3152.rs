struct Solution;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        for &x in &nums1 {
            if x % k != 0 {
                continue;
            }
            for &y in &nums2 {
                if x % (k * y) == 0 {
                    res += 1;
                }
            }
        }
        res
    }
}