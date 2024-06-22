fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
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