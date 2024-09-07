struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut f = HashMap::new();
        let mut mx = vec![0; k as usize + 2];
        for x in nums {
            if !f.contains_key(&x) {
                f.insert(x, vec![0; k as usize + 1]);
            }
            let cnt = f.get_mut(&x).unwrap();
            for i in (0..=k as usize).rev() {
                cnt[i] = std::cmp::max(cnt[i], mx[i]) + 1;
                mx[i + 1] = std::cmp::max(mx[i + 1], cnt[i]);
            }
        }
        mx.pop().unwrap()
    }
}