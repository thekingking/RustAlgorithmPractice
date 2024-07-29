struct Solution;

impl Solution {
    /// 差分
    pub fn shifting_letters(mut s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut cnt = vec![0; s.len() + 1];
        for s in shifts {
            let num = if s[2] == 1 { 1 } else { -1 };
            cnt[s[0] as usize] += num;
            cnt[s[1] as usize + 1] -= num;
        }
        let bs = unsafe {
            s.as_bytes_mut()
        };
        let mut sum = 0;
        for i in 0..bs.len() {
            sum = (sum + cnt[i]) % 26 + 26;
            bs[i] = (bs[i] - b'a' + sum as u8) % 26 + b'a';
        }
        s
    }
}
