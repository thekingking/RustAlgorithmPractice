struct Solution;

impl Solution {
    pub fn valid_strings(n: i32) -> Vec<String> {
        let mut res = vec![String::from("0"), String::from("1")];
        for _ in 1..n {
            let mut cnt = Vec::new();
            for s in res.iter_mut() {
                let l = s.pop().unwrap();
                if l == '0' {
                    cnt.push(s.clone() + "01");
                } else {
                    cnt.push(s.clone() + "10");
                    cnt.push(s.clone() + "11");
                }
            }
            res = cnt;
        }
        res
    }
}