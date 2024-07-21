struct Solution;

impl Solution {
    pub fn losing_player(x: i32, y: i32) -> String {
        let t = std::cmp::min(x, y / 4);
        if t % 2 == 1 {
            "Alice".to_string()
        } else {
            "Bob".to_string()
        }
    }
}
