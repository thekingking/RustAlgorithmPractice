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
    pub fn minimum_operations(mut nums: Vec<i32>, target: Vec<i32>) -> i64 {
        fn dfs(nums: &mut Vec<i32>, s: usize, e: usize) -> i64 {
            if s == e {
                return nums[s] as i64;
            }
            let mut min = i32::MAX;
            for &x in &nums[s..=e] {
                if min > x {
                    min = x;
                }
            }
            let mut res = min as i64;
            for i in s..=e {
                nums[i] -= min;
            }
            let mut l = s;
            let mut r = s;
            while r <= e && l <= e {
                while l <= e && nums[l] == 0 {
                    l += 1;
                }
                if l > e {
                    break;
                }
                r = l;
                while r <= e && nums[l] != 0 {
                    r += 1;
                }
                res += dfs(nums, l, r - 1);
                l = r;
            }
            res
        }

        for i in 0..nums.len() {
            nums[i] -= target[i];
        }
        let mut l = 0;
        let mut r = 0;
        let len = nums.len();
        let mut res = 0;
        while l < len && r < len {
            while l < len && nums[l] == 0 {
                l += 1;
            }
            if l == len {
                break;
            }
            r = l;
            if nums[l] > 0 {
                while r < len && nums[r] > 0 {
                    r += 1;
                }
            } else {
                while r < len && nums[r] < 0 {
                    nums[r] = - nums[r];
                    r += 1;
                }
            }
            res += dfs(&mut nums, l, r - 1);
            l = r;
        }
        res
    }
}
