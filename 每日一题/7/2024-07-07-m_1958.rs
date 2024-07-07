struct Solution;

impl Solution {
    /// 各个方向枚举
    pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
            let mut x = r_move + dx;
            let mut y = c_move + dy;
            if x < 0 || x >= m || y < 0 || y >= n || board[x as usize][y as usize] == '.' || board[x as usize][y as usize] == color {
                continue;
            }
            loop {
                x += dx;
                y += dy;
                if x < 0 || x >= m || y < 0 || y >= n || board[x as usize][y as usize] == '.' {
                    break;
                }
                if board[x as usize][y as usize] == color {
                    return true;
                }
            }
        }
        false
    }
}