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
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut cnt = std::collections::BTreeMap::new();
        for flower in flowers {
            cnt.entry(flower[0]).and_modify(|x| *x += 1).or_insert(1);
            cnt.entry(flower[1] + 1).and_modify(|x| *x -= 1).or_insert(-1);
        }
        let mut people: Vec<(i32, usize)> = std::iter::zip(people, 0..).collect();
        people.sort_unstable_by_key(|x| x.0);
        let mut sum = 0;
        let mut res = vec![0; people.len()];
        let mut i = 0;
        for (k, v) in cnt {
            while i < people.len() && people[i].0 < k {
                res[people[i].1] = sum;
                i += 1;
            }
            sum += v;
        }
        while i < people.len() {
            res[people[i].1] = sum;
            i += 1;
        }
        res
    }
}
