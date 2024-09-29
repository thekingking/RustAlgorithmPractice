struct Solution;

impl Solution {
    pub fn min_starting_index(s: String, pattern: String) -> i32 {
        fn cal_z(s: String) -> Vec<i32> {
            let n = s.len();
            let bs = s.as_bytes();
            let mut z = vec![0; n];
            let mut box_l = 0;
            let mut box_r = 0;
            for i in 1..n {
                if i <= box_r {
                    z[i] = std::cmp::min(z[i - box_l], (box_r - i + 1) as i32);
                }
                while (i + z[i] as usize) < n && bs[z[i] as usize] == bs[i + z[i] as usize] {
                    box_l = i;
                    box_r = i + z[i] as usize;
                    z[i] += 1;
                }
            }
            return z;
        }
        let pre_z = cal_z(pattern.clone() + &s);
        let mut p: Vec<char> = pattern.clone().chars().collect();
        p.reverse();
        let tp: String = p.into_iter().collect();
        let mut ts: Vec<char> = s.clone().chars().collect();
        ts.reverse();
        let ts: String = ts.into_iter().collect();
        let mut suf_z = cal_z(tp + &ts);
        suf_z.reverse();
        let m = pattern.len();
        for i in m..s.len() + 1 {
            if pre_z[i] + suf_z[i - 1] >= m as i32 - 1 {
                return (i - m) as i32;
            }
        }
        -1
    }
}
