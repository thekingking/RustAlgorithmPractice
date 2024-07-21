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
        let mut cnt = std::collections::HashMap::new();
        let len = nums.len();
        for i in 0..len / 2 {
            cnt.entry((nums[i] - nums[len - i - 1]).abs()).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut res = i32::MAX;
        
        while cnt.len() != 0 {
            let mut max = 0;
            let mut num = 0;
            for (&k, &v) in &cnt {
                if v > num {
                    max = k;
                    num = v;
                }
            }
            if max <= k / 2 {
                return res.min(len as i32 / 2 - num);
            } else {
                cnt.remove(&max);
                let mut n = 0;
                for i in 0..len / 2 {
                    if (nums[i] - nums[len - i - 1]).abs() == max {
                        continue;
                    }
                    if nums[i] + max <= k || nums[i] - max >= 0 || nums[len - i - 1] + max <= k || nums[len - i - 1] - max >= 0 {
                        n += 1;
                    } else {
                        n += 2;
                    }
                }
                res = res.min(n);
            }
        }

        res
    }
}
