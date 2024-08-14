struct Solution;

impl Solution {
    pub fn survived_robots_healths(positions: Vec<i32>, healths: Vec<i32>, directions: String) -> Vec<i32> {
        let mut cnt = Vec::new();
        let bs = directions.into_bytes();
        for i in 0..positions.len() {
            cnt.push((i, positions[i], healths[i], bs[i]));
        }
        cnt.sort_unstable_by_key(|row| row.1);
        let mut stack: Vec<(usize, i32, i32, u8)> = Vec::new();
        for row in cnt {
            let mut flag = true;
            let mut cur = row;
            while stack.len() > 0 && stack.last().unwrap().3 == b'R' && cur.3 == b'L' {
                let mut pre = stack.pop().unwrap();
                if pre.2 > cur.2 {
                    pre.2 -= 1;
                    cur = pre;
                    break;
                } else if pre.2 == cur.2 {
                    flag = false;
                    break;
                } else {
                    cur.2 -= 1;
                }
            }
            if flag {
                stack.push(cur);
            }
        }
        stack.sort_unstable_by_key(|row| row.0);
        stack.into_iter().map(|row| row.2).collect()
    }
}
