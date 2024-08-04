struct Solution;

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let mut cnt = vec![vec![0; 11]; n as usize + 1];
        let mut res = 0;
        for p in pick {
            cnt[p[0] as usize][p[1] as usize] += 1;
        }
        for (i, row) in cnt.into_iter().enumerate() {
            let mut max = 0;
            for x in row {
                if x > max {
                    max = x;
                }
            }
            if max as usize > i {
                res += 1;
            }
        }
        res
    }
}