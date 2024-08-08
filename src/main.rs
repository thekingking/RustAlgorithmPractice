
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

struct MinStack {
    stack: Vec<i32>,
    min_pre: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min_pre: vec![i32::MAX],
        }
    }
    
    fn push(&mut self, val: i32) {
        self.min_pre.push(std::cmp::min(val, *self.min_pre.last().unwrap()));
        self.stack.push(val);
    }
    
    fn pop(&mut self) {
        self.min_pre.pop();
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.min_pre.last().unwrap()
    }
}