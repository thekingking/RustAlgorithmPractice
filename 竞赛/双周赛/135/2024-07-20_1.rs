struct Solution;

impl Solution {
    /// 题解
    pub fn losing_player(x: i32, y: i32) -> String {
        if std::cmp::min(x, y / 4) % 2 == 1 { "Alice".to_string() } else { "Bob".to_string() }
    }

    /// 周赛
    pub fn losing_player(x: i32, y: i32) -> String {
        let t = std::cmp::min(x, y / 4);
        if t % 2 == 1 {
            "Alice".to_string()
        } else {
            "Bob".to_string()
        }
    }
}
