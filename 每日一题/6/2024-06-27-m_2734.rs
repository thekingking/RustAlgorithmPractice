struct Solution;

impl Solution {
    /// 贪心
    pub fn smallest_string(mut s: String) -> String {
        let mut bytes = unsafe {
            s.as_bytes_mut()
        };
        let mut index = 0;
        while index < bytes.len() && bytes[index] == b'a' {
            index += 1;
        }
        if index == bytes.len() {
            bytes[index - 1] = b'z';
        } else {
            while index < bytes.len() && bytes[index] != b'a' {
                bytes[index] -= 1;
                index += 1;
            }
        }
        s
    }
}