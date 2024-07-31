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
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut diff = vec![vec![0; n + 1]; n + 1];
        for q in queries {
            let x1 = q[0] as usize;
            let y1 = q[1] as usize;
            let x2 = q[2] as usize;
            let y2 = q[3] as usize;
            diff[x1][y1] += 1;
            diff[x1][y2 + 1] -= 1;
            diff[x2 + 1][y1] -= 1;
            diff[x2 + 1][y2 + 1] += 1;
        }
        for i in 1..=n {
            for j in 1..=n {
                diff[i][j] += diff[i - 1][j] + diff[i][j - 1] - diff[i - 1][j - 1];
            }
        }
        let mut mat = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                mat[i][j] = diff[i + 1][j + 1];
            }
        }
        mat
    }
}


struct NumMatrix {
    sum: Vec<Vec<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut cnt = vec![vec![0; m + 1]; n + 1];
        for i in 1..=n {
            for j in 1..=m {
                cnt[i][j] = cnt[i - 1][j] + cnt[i][j - 1] - cnt[i - 1][j - 1] + matrix[i - 1][j - 1];
            }
        }
        Self {
            sum: cnt,
        }
    }
    
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        self.sum[row2 + 1][col2 + 1] - self.sum[row1][col2 + 1] - self.sum[row2 + 1][col1] + self.sum[row1][col1]
    }
}

