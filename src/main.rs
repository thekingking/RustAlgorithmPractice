

fn main() {
    println!("hello, world");
}

struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        use std::collections::HashMap;
        use std::collections::BinaryHeap;
        use std::collections::HashSet;
        
        let mut map = HashMap::new();
        let mut set = HashSet::new();
        let mut res = vec![];
        for i in 0..k as usize {
            *map.entry(nums[i]).or_insert(0) += 1;
        }
        let mut heap = BinaryHeap::from_iter(map.iter().map(|(&k, &v)| (v, k)));
        let mut sum = 0;
        for i in 0..x {
            if let Some((v, k)) = heap.pop() {
                sum += k as i64 * v as i64;
                set.insert(k);
            }
        }
        res.push(sum);
        for l in 1..nums.len() - k as usize + 1 {
            let r = l + k as usize - 1;
            map.entry(nums[l - 1]).and_modify(|v| *v -= 1);
            map.entry(nums[r]).and_modify(|v| *v += 1).or_insert(1);
            
        }
        res
    }
}