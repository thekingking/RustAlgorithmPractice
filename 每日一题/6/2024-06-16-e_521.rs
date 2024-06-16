struct Solution;

impl Solution {
    /// 一看题目挺唬人的，实际还是一个简单题
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a.len() != b.len() {
            return if a.len() > b.len() { a.len() as i32 } else { b.len() as i32};
        }
        let a_byte = a.as_bytes();
        let b_byte = b.as_bytes();
        let len = a.len();
        for i in 0..len {
            if a_byte[i] != b_byte[i] {
                return len as i32
            }
        }
        -1
    }
}
