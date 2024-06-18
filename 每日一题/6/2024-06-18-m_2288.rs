struct Solution;

impl Solution {
    /// 模拟，按着题目写就行了
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        let mut words: Vec<String> = sentence.split(' ').map(|s| s.to_string()).collect();
        'next: for i in 0..words.len() {
            let b = words[i].as_bytes();
            if b.len() > 1 && b[0] == b'$' {
                let mut num: i64 = 0;

                for &c in &b[1..] {
                    if c >= b'0' && c <= b'9' {
                        num = num * 10 + (c - b'0') as i64;
                    } else {
                        continue 'next;
                    }
                }
                let s = ((num * (100 - discount as i64)) as f64) / 100.0;
                words[i] = format!("${:.2}", s);
            }
        }
        words.join(" ")
    }
}