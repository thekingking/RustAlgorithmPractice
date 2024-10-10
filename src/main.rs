use std::borrow::BorrowMut;
use std::{i32, path, vec};



fn main() {
    println!("hello, world");
}

struct Solution;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        for &x in &nums1 {
            if x % k != 0 {
                continue;
            }
            for &y in &nums2 {
                if x % (k * y) == 0 {
                    res += 1;
                }
            }
        }
        res
    }
}

// use std::collections::HashMap;
// use std::cell::RefCell;
// use std::rc::Rc;

// struct Node {
//     key: i32,
//     value: i32,
//     prev: Option<Rc<RefCell<Node>>>,
//     next: Option<Rc<RefCell<Node>>>,
// }

// impl Node {
//     fn new(key: i32, value: i32) -> Rc<RefCell<Self>> {
//         Rc::new(RefCell::new(Node {
//             key,
//             value,
//             prev: None,
//             next: None
//         }))
//     }
// }

// struct LRUCache {
//     capacity: usize,
//     dummy: Rc<RefCell<Node>>,
//     key_to_node: HashMap<i32, Rc<RefCell<Node>>>,
// }


// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl LRUCache {

//     fn new(capacity: i32) -> Self {
//         // 哨兵
//         let dummy = Node::new(0, 0);
//         dummy.borrow_mut().prev = Some(dummy.clone());
//         dummy.borrow_mut().next = Some(dummy.clone());
//         LRUCache {
//             capacity: capacity as usize,
//             dummy,
//             key_to_node: HashMap::new(),
//         }
//     }
    
//     fn get(&mut self, key: i32) -> i32 {
//         if let Some(node) = self.key_to_node.get(&key) {
//             let node = node.clone();
//             let value = node.borrow().value;
//             self.remove(node.clone());
//             self.push_front(node);
//             return value;
//         }
//         -1
//     }
    
//     fn put(&mut self, key: i32, value: i32) {
//         if let Some(node) = self.key_to_node.get(&key) {
//             let node = node.clone();
//             node.borrow_mut().value = value;
//             self.remove(node.clone());
//             self.push_front(node);
//             return;
//         }
//         let node = Node::new(key, value);
//         self.key_to_node.insert(key, node.clone());
//         self.push_front(node);
//         if self.key_to_node.len() > self.capacity {
//             let back_node = self.dummy.borrow().prev.clone().unwrap();
//             self.key_to_node.remove(&back_node.borrow().key);
//             self.remove(back_node);
//         }
//     }

//     fn remove(&mut self, x: Rc<RefCell<Node>>) {
//         let prev = x.borrow().prev.clone().unwrap();
//         let next = x.borrow().next.clone().unwrap();
//         prev.borrow_mut().next = Some(next.clone());
//         next.borrow_mut().prev = Some(prev);
//     }

//     fn push_front(&mut self, x: Rc<RefCell<Node>>) {
//         let next = self.dummy.borrow().next.clone().unwrap();
//         x.borrow_mut().prev = Some(self.dummy.clone());
//         x.borrow_mut().next = Some(next.clone());
//         self.dummy.borrow_mut().next = Some(x.clone());
//         next.borrow_mut().prev = Some(x);
//     }
// }