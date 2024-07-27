struct Solution;

impl Solution {
    ///  贪心
    pub fn get_smallest_string(mut s: String, mut k: i32) -> String {
        let bs = unsafe {
            s.as_bytes_mut()
        };
        for b in bs {
            let dis = std::cmp::min(*b - b'a', b'z' - *b + 1) as i32;
            if dis <= k {
                k -= dis;
                *b = b'a';
            } else {
                *b -= k as u8;
                break;
            }
        }
        s
    }
}
