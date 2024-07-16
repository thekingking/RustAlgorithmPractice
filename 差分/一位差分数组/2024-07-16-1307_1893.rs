struct Solution;

impl Solution {
    /// 简单差分
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut cnt = vec![0; 52];
        for r in ranges {
            cnt[r[0] as usize] += 1;
            cnt[r[1] as usize + 1] -= 1;
        }
        let mut num = 0;
        for (i, &x) in cnt.iter().enumerate() {
            num += x;
            if i >= left as usize && i <= right as usize && num <= 0 {
                return false;
            }
        }
        true
    }
}
