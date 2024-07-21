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
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = vec![0; k as usize + 1];
        let mut cnt2 = vec![0; k as usize + 1];
        let n = nums.len();
        for i in 0..n / 2 {
            let (mut p, mut q) = (nums[i], nums[n - i - 1]);
            if p > q {
                (p, q) = (q, p);
            }
            cnt[(q - p) as usize] += 1;
            cnt2[std::cmp::max(q, k - p) as usize] += 1;
        }
        let mut ans = n as i32;
        // 范围之外，需要改动两次的数目
        let mut sum2 = 0;
        // 枚举所有可能
        for i in 0..=(k as usize) {
            ans = std::cmp::min(ans, n as i32 / 2 - cnt[i] + sum2);
            sum2 += cnt2[i];
        }
        ans
    }
}
