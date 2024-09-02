struct Solution;

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let n = answer_key.len();
        let bs = answer_key.as_bytes();
        let mut res = 0;
        let mut cnt = 0;
        let mut left = 0;
        let mut right = 0;
        while right < n {
            while right < n && !(cnt == k && bs[right] == b'F') {
                if bs[right] == b'F' {
                    cnt += 1;
                }
                right += 1;
            }
            if right - left > res {
                res = right - left;
            }
            if bs[left] == b'F' {
                cnt -= 1;
            }
            left += 1;
        }
        cnt = 0;
        left = 0;
        right = 0;
        while right < n {
            while right < n && !(cnt == k && bs[right] == b'T') {
                if bs[right] == b'T' {
                    cnt += 1;
                }
                right += 1;
            }
            if right - left > res {
                res = right - left;
            }
            if bs[left] == b'T' {
                cnt -= 1;
            }
            left += 1;
        }
        res as i32
    }
}