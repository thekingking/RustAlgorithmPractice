struct Solution;

impl Solution {
    /// 双百hard
    /// 贪心
    pub fn smallest_beautiful_string(mut s: String, k: i32) -> String {
        let s_byte = unsafe {
            s.as_bytes_mut()
        };
        for i in (0..s_byte.len()).rev() {
            let mut b = s_byte[i] + 1;
            if i > 0 && s_byte[i - 1] == b || i > 1 && s_byte[i - 2] == b {
                b += 1;
            }
            if i > 0 && s_byte[i - 1] == b || i > 1 && s_byte[i - 2] == b {
                b += 1;
            }
            if b >= b'a' + k as u8 {
                continue;
            }
            s_byte[i] = b;
            let mut j = i + 1;
            while j < s_byte.len() {
                let mut b = b'a';
                if j > 0 && s_byte[j - 1] == b || j > 1 && s_byte[j - 2] == b {
                    b += 1;
                }
                if j > 0 && s_byte[j - 1] == b || j > 1 && s_byte[j - 2] == b {
                    b += 1;
                }
                s_byte[j] = b;
                j += 1;
            }
            return s;
        }
        String::from("")
    }

    /// 灵神题解
    /// 双百 贪心
    pub fn smallest_beautiful_string(mut s: String, k: i32) -> String {
        let s_byte = unsafe {
            s.as_bytes_mut()
        };
        let n = s_byte.len();
        let k = k as u8 + b'a';
        let mut i = n - 1;
        s_byte[i] += 1;
        while i < n {
            if s_byte[i] == k {
                if i == 0 {
                    return String::from("");
                }
                s_byte[i] = b'a';
                i -= 1;
                s_byte[i] += 1;
            } else if i > 0 && s_byte[i] == s_byte[i - 1] || i > 1 && s_byte[i] == s_byte[i -  2] {
                s_byte[i] += 1;
            } else {
                i += 1;
            }
        }
        s
    }
}