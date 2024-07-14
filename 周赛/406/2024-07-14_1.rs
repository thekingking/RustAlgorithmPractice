struct Solution;

impl Solution {
    pub fn get_smallest_string(mut s: String) -> String {
        let bs = unsafe {
            s.as_bytes_mut()
        };
        let mut i = 0;
        while i + 1 < bs.len() {
            if bs[i] > bs[i + 1] && (bs[i] - bs[i + 1]) % 2 == 0 {
                let t = bs[i];
                bs[i] = bs[i + 1];
                bs[i + 1] = t;
                break;
            } 
            i += 1;
        }
        s
    }
}