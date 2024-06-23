fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let bytes = word.as_bytes();
        bytes.iter().all(|&x| x >= b'A' && x <= b'Z') || bytes[1..].iter().all(|&x| x >= b'a' && x <= b'z')
    }
}