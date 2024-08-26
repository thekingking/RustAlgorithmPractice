struct Solution;

impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        let mut stack = Vec::new();
        for i in 0..n {
            match i % 4 {
                0 => stack.push(n - i),
                1 => *stack.last_mut().unwrap() *= n - i,
                2 => *stack.last_mut().unwrap() /= n - i,
                3 => stack[0] += n - i,
                _ => (),
            }
        }
        stack.into_iter().reduce(|acc, e| acc - e).unwrap()
    }
}