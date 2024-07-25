use std::{cmp, collections::{HashMap, HashSet}, i32, vec};

fn main() {
    println!("hello, world");
}

#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    value: bool,
}

#[derive(Default)]
struct WordDictionary {
    root: Node,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
        Self::default()
    }
    
    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;
        for &b in word.as_bytes() {
            let idx = (b - b'a') as usize;
            let next = &mut node.children[idx];
            node = next.get_or_insert_with(Box::<Node>::default);
        }
        node.value = true;
    }
    
    fn search(&self, word: String) -> bool {
        fn search(mut node: &Node, word: &str) -> bool {
            if word.len() == 0 {
                return node.value
            }
            let b = word.as_bytes()[0];
            if b == b'.' {
                for i in 0..26 {
                    if let Some(next) = &node.children[i] {
                        if search(&next, &word[1..]) {
                            return true;
                        }
                    }
                }
            } else {
                if let Some(next) = &node.children[(b - b'a') as usize] {
                    return search(&next, &word[1..]);
                }
            }
            false
        }
        search(&self.root, &word)
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */

struct Solution;

impl Solution {
    pub fn minimum_operations(num: String) -> i32 {
        let mut res1 = 0;
        let mut res2 = 0;
        let bs = num.as_bytes();
        let n = bs.len();
        while res1 < n && bs[n - res1 - 1] != b'5' {
            res1 += 1;
        }
        res1 += 1;
        while res1 < n && bs[n - res1 - 1] != b'2' && bs[n - res1 - 1] != b'7' {
            res1 += 1;
        }
        if res1 == n {
            res1 += 1;
        }
        while res2 < n && bs[n - res2 - 1] != b'0' {
            res2 += 1;
        }
        res2 += 1;
        while res2 < n && bs[n - res2 - 1] != b'0' && bs[n - res2 - 1] != b'5' {
            res2 += 1;
        }
        n.min(res1 - 1).min(res2 - 1) as i32
    }
}
