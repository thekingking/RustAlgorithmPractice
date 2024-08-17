struct Solution;

impl Solution {
    pub fn maximum_value_sum(board: Vec<Vec<i32>>) -> i64 {
        let m = board.len();
        let n = board[0].len();
        let mut cnt = Vec::new();
        for i in 0..m {
            for j in 0..n {
                cnt.push((i, j, board[i][j] as i64));
            }
        }
        cnt.sort_unstable_by_key(|x| -x.2);
        let mut row = vec![false; m];
        let mut col = vec![false; n];
        fn dfs(cnt: &Vec<(usize, usize, i64)>, board: &Vec<Vec<i32>>, row: &mut Vec<bool>, col: &mut Vec<bool>, num: i32) -> i64 {
            if num == 2 {
                for i in 0..cnt.len() {
                    if row[cnt[i].0] || col[cnt[i].1] {
                        continue;
                    }
                    return cnt[i].2;
                }
                return i32::MIN as i64 * 10;
            }
            let mut res = i32::MIN as i64 * 10;
            let mut k = 0;
            while k < cnt.len() {
                if !row[cnt[k].0] && !col[cnt[k].1] {
                    break;
                }
                k += 1;
            }
            let r = cnt[k].0;
            let c = cnt[k].1;
            for j in 0..col.len() {
                if row[r] || col[j] {
                    continue;
                }
                row[r] = true;
                col[j] = true;
                res = res.max(board[r][j] as i64 + dfs(cnt, board, row, col, num + 1));
                row[r] = false;
                col[j] = false;
            }
            for i in 0..row.len() {
                if row[i] || col[c] {
                    continue;
                }
                row[i] = true;
                col[c] = true;
                res = res.max(board[i][c] as i64 + dfs(cnt, board, row, col, num + 1));
                row[i] = false;
                col[c] = false;
            }
            res
        }
        dfs(&cnt, &board, &mut row, &mut col, 0)
    }
}
