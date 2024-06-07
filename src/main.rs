use std::collections::HashMap;

fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
        let mut cnt = HashMap::new();
        let mut sum = 0;
        let mut left = 0;
        let mut right = 0;
        for (r, x) in array.iter().enumerate() {
            cnt.entry(sum).or_insert(r);
            let &bit = x.as_bytes().first().unwrap();
            if bit >= b'0' && bit <= b'9' {
                sum += 1;
            } else {
                sum -= 1;
            }
            let &l = cnt.get(&sum).unwrap_or(&(r + 1));
            if r + 1 - l > right - left {
                right = r + 1;
                left = l;
            }
        }
        array[left..right].to_owned()
    }
}
