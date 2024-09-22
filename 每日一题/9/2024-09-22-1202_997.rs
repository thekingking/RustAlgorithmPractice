struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut cnt = vec![0; n as usize];
        for arr in trust {
            cnt[arr[1] as usize - 1] += 1;
            cnt[arr[0] as usize - 1] -= 1;
        }
        for (i, x) in cnt.into_iter().enumerate() {
            if x == n - 1 {
                return i as i32 + 1;
            }
        }
        -1
    }
}