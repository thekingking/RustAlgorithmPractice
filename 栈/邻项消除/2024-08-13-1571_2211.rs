struct Solution;

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let mut pre = b'L';
        let mut num = 0;
        let mut res = 0;
        for &c in directions.as_bytes() {
            if pre == b'R' && c == b'L' {
                res += num + 1;
                pre = b'S';
                num = 0;
            } else if pre == b'R' && c == b'S' {
                res += num;
                pre = b'S';
                num = 0;
            } else if pre == b'S' && c == b'L' {
                res += 1;
                pre = b'S';
            } else if c == b'R' {
                num += 1;
                pre = c;
            } else {
                pre = c;
            }
        }
        res
    }
}
