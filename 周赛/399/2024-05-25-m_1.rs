struct Solution;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut answer = 0;
        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                if nums1[i] % (nums2[j] * k) == 0 {
                    answer += 1;
                }
            }
        }
        answer
    }
}