struct Solution;

impl Solution {
    pub fn get_encrypted_string(s: String, k: i32) -> String {
        let mut left = s;
        let right = left.split_off(k as usize % left.len());
        right + &left
    }
}