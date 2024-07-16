struct Solution;

impl Solution {
    /// 哈希表
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut cnt1 = [false; 101];
        let mut cnt2 = [false; 101];
        let mut res1 = 0;
        let mut res2 = 0;
        for &x in &nums1 {
            cnt1[x as usize] = true;
        }
        for &x in &nums2 {
            cnt2[x as usize] = true;
        }
        for &x in &nums1 {
            res1 += cnt2[x as usize] as i32;
        }
        for &x in &nums2 {
            res2 += cnt1[x as usize] as i32;
        }
        vec![res1, res2]
    }
}
