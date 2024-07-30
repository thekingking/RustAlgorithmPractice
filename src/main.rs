use std::{cmp::{self, max}, collections::{BTreeMap, HashMap, HashSet}, i32, vec};

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

 struct MyCalendarThree {
    bt: BTreeMap<i32, i32>,
 }
 
 
 /**
  * `&self` means the method takes an immutable reference.
  * If you need a mutable reference, change it to `&mut self` instead.
  */
 impl MyCalendarThree {
 
     fn new() -> Self {
        Self {
            bt: BTreeMap::new(),
        }
     }
     
     fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        self.bt.entry(start_time).and_modify(|x| *x += 1).or_insert(1);
        self.bt.entry(end_time).and_modify(|x| *x -= 1).or_insert(-1);
        let mut res = 1;
        let mut sum = 0;
        for &v in self.bt.values() {
            sum += v;
            res = res.max(sum)
        }
        res
     }
 }
 
 /**
  * Your MyCalendarThree object will be instantiated and called as such:
  * let obj = MyCalendarThree::new();
  * let ret_1: i32 = obj.book(startTime, endTime);
  */

struct Solution;

impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut cnt = vec![0; n + 1];
        for (i, &x) in nums.iter().enumerate() {
            let x = x as usize;
            if i >= x {
                cnt[0] += 1;
                cnt[i - x + 1] -= 1;
                cnt[i + 1] += 1;
                cnt[n] -= 1;

            } else {
                cnt[i + 1] += 1;
                cnt[n - x + i + 1] -= 1;
            }
        }
        let mut sum = 0;
        let mut res = 0;
        let mut max = 0;
        for (i, &c) in cnt.iter().enumerate() {
            sum += c;
            if sum > max {
                max = sum;
                res = i;
            }
        }
        res as i32
    }
}
