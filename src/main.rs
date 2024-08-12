use std::{default, fmt::Debug};



fn main() {
    println!("hello, world");
}

struct Solution;

impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut dp = vec![0; 1002];
        for i in 0..52 {
            dp[i as usize] = i;
        }
        let mut pre = nums[0];
        for i in 1..nums.len() {
            let s = (nums[i] - pre).max(0) as usize;
            let mut new_dp = vec![0; 1002];
            for j in s..=nums[i] as usize {
                new_dp[j + 1] = (dp[j + 1 - s] + new_dp[j]) % MOD;
            }
            pre = nums[i];
            dp = new_dp;
        }
        dp[*nums.last().unwrap() as usize + 1] as i32
    }
}


#[derive(Debug, Default)]
struct Node<Type> {
    children: [Option<Box<Node<Type>>>; 26],
    value: Option<Type>,
}

#[derive(Debug, Default)]
struct Trie<Type>
where 
    Type: Default + Copy
{
    root: Node<Type>,
}

impl<Type> Trie<Type>
where  
    Type: Default + Copy,
{
    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    pub fn insert(&mut self, key: String, value: Type)
    {
        let mut node = &mut self.root;
        for &c in key.as_bytes() {
            let next = &mut node.children[(c - b'a') as usize];
            node = next.get_or_insert_with(Box::<Node<Type>>::default);
        }
        node.value = Some(value);
    } 

    pub fn get(&self, key: String) -> Option<Type> 
    where
    {
        let mut node = &self.root;
        for c in key.as_bytes() {
            if let Some(next) = &node.children[(c - b'a') as usize] {
                node = next.as_ref();
            } else {
                return None;
            }
        }
        node.value
    }
}


struct MagicDictionary {
    trie: Trie<bool>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {

    fn new() -> Self {
        Self {
            trie: Trie::default(),
        }
    }
    
    fn build_dict(&mut self, dictionary: Vec<String>) {
        for dic in dictionary {
            self.trie.insert(dic, true)
        }
    }
    
    fn search(&self, search_word: String) -> bool {
        for i in 0..search_word.len() {
            let mut new_string = search_word.clone();
            let pre = new_string.remove(i) as u8;
            for j in 0..26 {
                if j == pre - b'a' {
                    continue;
                }
                let mut s = new_string.clone();
                s.insert(i, (j + b'a') as char);
                if let Some(_) = self.trie.get(s) {
                    return true;
                }
            }
        }
        false
    }
}
