struct Solution;

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut res = vec![0; n as usize];
        let mut stack = Vec::new();
        let mut pre = 0;
        for s in logs {
            let bs: Vec<&str> = s.split(':').collect();
            let id = bs[0].parse::<usize>().unwrap();
            let t = bs[2].parse::<i32>().unwrap();
            if let Some(&i) = stack.last() {
                res[i] += t - pre;
            }
            if bs[1] == "start" {
                stack.push(id);
                pre = t;
            } else {
                res[stack.pop().unwrap()] += 1;
                pre = t + 1;
            }
        }   
        res
    }
}
