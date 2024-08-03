struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let bs = path.into_bytes();
        let mut res = Vec::new();
        let mut i = 0;
        let n = bs.len();
        while i < n {
            while i < n && bs[i] == b'/' {
                i += 1;
            }
            let mut cnt = Vec::new();
            while i < n && bs[i] != b'/' {
                cnt.push(bs[i]);
                i += 1;
            }
            if cnt.len() == 0 {
                break;
            }
            if cnt.len() == 2 && cnt[0] == b'.' && cnt[1] == b'.' {
                res.pop();
            } else if !(cnt.len() == 1 && cnt[0] == b'.') {
                res.push(String::from_utf8(cnt).unwrap());
            }
        }
        "/".to_string() + &res.join("/")
    }
}