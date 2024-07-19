struct Solution;

impl Solution {
    /// 前缀和
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let mut sum = 0;
        for &p in &possible {
            if p == 1 {
                sum += 1;
            } else {
                sum -= 1;
            }
        }
        let mut res = 0;
        for (i, &p) in possible[0..(possible.len() - 1)].iter().enumerate() {
            if p == 1 {
                res += 1;
            } else {
                res -= 1;
            }
            if res * 2 > sum {
                return i as i32 + 1;
            }
        }
        -1
    }
}
