
fn main() {
    println!("hello, world");
}

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

struct CustomStack {
    max_size: usize,
    stack: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {

    fn new(maxSize: i32) -> Self {
        Self {
            max_size: maxSize as usize,
            stack: Vec::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.stack.len() < self.max_size {
            self.stack.push(x);
        }
    }
    
    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap_or(-1)
    }
    
    fn increment(&mut self, k: i32, val: i32) {
        for i in 0..self.stack.len().min(k as usize) {
            self.stack[i] += val;
        }
    }
}