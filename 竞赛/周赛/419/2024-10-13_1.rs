struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        use std::collections::HashMap;
        use std::collections::BinaryHeap;
        let mut map = HashMap::new();
        let mut res = vec![];
        let mut r = 0;
        for l in 0..nums.len() - k as usize + 1 {
            let mut cnt = 0;
            while r < l + k as usize {
                *map.entry(nums[r]).or_insert(0) += 1;
                r += 1;
            }
            let mut heap = BinaryHeap::from_iter(map.iter().map(|(&k,&v)| (v, k)));
            let mut num = 0;
            while let Some((v, k)) = heap.pop() {
                if num == x {
                    break;
                }
                cnt += k * v;
                num += 1;
            }
            res.push(cnt);
            *map.get_mut(&nums[l]).unwrap() -= 1;
            if map[&nums[l]] == 0 {
                map.remove(&nums[l]);
            }
        }
        res
    }
}