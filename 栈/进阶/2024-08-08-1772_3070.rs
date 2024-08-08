struct Solution;

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut cnt = vec![vec![]; 26];
        for (i, &c) in s.as_bytes().iter().enumerate() {
            if c == b'*' {
                for j in 0..26 {
                    if let Some(_) = cnt[j].pop() {
                        break;
                    }
                }
            } else {
                cnt[(c - b'a') as usize].push(i);
            }
        }
        let mut res = Vec::new();
        for i in (0..s.len()).rev(){
            for j in 0..26 {
                if let Some(&x) = cnt[j].last() {
                    if x == i {
                        cnt[j].pop();
                        res.push(j as u8 + b'a');
                        break;
                    }
                }
            }
        }
        res.reverse();
        unsafe {String::from_utf8_unchecked(res)}
    }
}
