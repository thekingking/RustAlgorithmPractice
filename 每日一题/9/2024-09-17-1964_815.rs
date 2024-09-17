struct Solution;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        use std::collections::HashSet;
        let mut vis = vec![false; routes.len()];
        let mut num = 0;
        let mut queue = HashSet::new();
        queue.insert(source);
        while queue.len() > 0 { 
            num += 1;
            let mut tmp = HashSet::new();
            for i in 0..routes.len() {
                if vis[i] {
                    continue;
                }
                let mut flag = false;
                for &r in &routes[i] {
                    if queue.contains(&r) {
                        flag = true;
                    }
                }
                if flag {
                    for &r in &routes[i] {
                        if r == target {
                            return num;
                        }
                        tmp.insert(r);
                    }
                }
                vis[i] = true;
            }
            queue = tmp;
        }
        -1
    }
}