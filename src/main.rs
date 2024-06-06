fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
  pub fn minimum_steps(s: String) -> i64 {
    let mut num = 0;
    let mut ans = 0;
    for (i, c) in s.chars().enumerate() {
      if c == '0' {
        ans += i as i64 - num;
        num += 1;
      }
    }
    ans
  }
}