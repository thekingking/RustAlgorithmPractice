use std::{cmp, collections::{HashMap, HashSet}, vec};

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
    pub fn maximum_detonation(mut bombs: Vec<Vec<i32>>) -> i32 {
        let n = bombs.len();
        let mut g = vec![0i128; n];
        for i in 0..n {
            for j in 0..n {
                let dis = ((bombs[i][0] - bombs[j][0]) as i64).pow(2) + ((bombs[i][1] - bombs[j][1]) as i64).pow(2);
                if dis <= (bombs[i][2] as i64).pow(2) {
                    g[i] |= 1 << j;
                }
            }
        }
        for k in 0..n {
            for i in 0..n {
                if g[i] >> k & 1 != 0 {
                    g[i] |= g[k];
                }
            }
        }
        let mut res = 0;
        for x in g {
            res = res.max(x.count_ones() as i32);
        }
        res
    }
}
