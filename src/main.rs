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
    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let mut cnt = [[i32::MAX, i32::MAX]; 26];
        let bs = s.into_bytes();
        let n  = points.len();
        for i in 0..n {
            let dis = std::cmp::max(points[i][0].abs(), points[i][1].abs());
            let bit = (bs[i] - b'a') as usize;
            if cnt[bit][0] > dis {
                cnt[bit][1] = cnt[bit][0];
                cnt[bit][0] = dis;
            } else if cnt[bit][1] > dis {
                cnt[bit][1] = dis;
            }
        }
        let mut min = i32::MAX;
        for i in 0..26 {
            if min > cnt[i][1] {
                min = cnt[i][1];
            }
        }
        let mut res = 0;
        for i in 0..26 {
            if cnt[i][0] < min {
                res += 1;
            }
        }
        res
    }
}